#!/usr/bin/env python3
"""Local desktop model explorer for the existing ZFree provider toolkit.

A read-only Tkinter GUI. No browser, inference, account settings, or secrets are
sent to the model catalogue endpoints. Uses the existing ZFree discovery logic.
"""
from __future__ import annotations
import csv
import io
import json
import pathlib
import queue
import threading
import tkinter as tk
from tkinter import filedialog, messagebox, ttk
import zfree
import opencode_native

URL = 'http://127.0.0.1:8765/v1'
COLS = ('provider', 'name', 'id', 'context', 'route', 'evidence')


def as_text(value):
    return '' if value is None else str(value)


def normalize_models(gateway, opencode=()):
    models = [dict(m) for m in gateway if isinstance(m, dict) and isinstance(m.get('id'), str)]
    for item in opencode:
        if isinstance(item, dict) and isinstance(item.get('id'), str):
            models.append({**item, 'provider': 'opencode-native',
                           'route_id': None, 'context_length': 0,
                           'evidence': item.get('free_evidence', 'native-only'),
                           'routable': False})
    for m in models:
        m.setdefault('route_id', m.get('id'))
        m.setdefault('routable', m['provider'] != 'opencode-native')
        m.setdefault('evidence', 'catalog')
        m.setdefault('name', m['id'])
    return sorted(models, key=lambda m: (m.get('provider', ''), m['id']))


def filtered(models, term='', provider='All providers'):
    needle = term.casefold().strip()
    return [m for m in models
            if (provider == 'All providers' or m.get('provider') == provider)
            and (not needle or needle in ' '.join(as_text(m.get(k)) for k in
                    ('id', 'route_id', 'name', 'description', 'evidence')).casefold())]


class Explorer(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title('ZFree · Free Model Explorer')
        self.geometry('1190x790')
        self.minsize(850, 570)
        self.configure(bg='#101824')
        self.models = []
        self.visible = []
        self.events = queue.Queue()
        self.status = tk.StringVar(value='Ready · cached inventory shown until refreshed')
        self.search = tk.StringVar()
        self.provider = tk.StringVar(value='All providers')
        self.selected = None
        self._style()
        self._layout()
        self._load_cache()
        self.after(100, self._pump)
        self.search.trace_add('write', lambda *_: self._render())
        self.provider.trace_add('write', lambda *_: self._render())

    def _style(self):
        style = ttk.Style(self)
        if 'clam' in style.theme_names():
            style.theme_use('clam')
        style.configure('.', font=('Segoe UI', 10), background='#101824', foreground='#e6edf6')
        style.configure('TFrame', background='#101824')
        style.configure('TLabel', background='#101824', foreground='#d8e3f1')
        style.configure('Header.TLabel', font=('Segoe UI Semibold', 19), foreground='#f6f9fd')
        style.configure('Sub.TLabel', foreground='#91a9c4')
        style.configure('TButton', padding=(11, 7), background='#20314a', foreground='#ecf4ff')
        style.map('TButton', background=[('active', '#315075')])
        style.configure('TEntry', fieldbackground='#172639', foreground='#f6f9fd', insertcolor='#ffffff')
        style.configure('TCombobox', fieldbackground='#172639', foreground='#f6f9fd')
        style.configure('Treeview', background='#172639', foreground='#e6edf6', fieldbackground='#172639', rowheight=28, borderwidth=0)
        style.configure('Treeview.Heading', background='#263a57', foreground='#f8fbff', font=('Segoe UI Semibold', 10))
        style.map('Treeview', background=[('selected', '#315e83')])

    def _layout(self):
        header = ttk.Frame(self, padding=(20, 17, 20, 10)); header.pack(fill='x')
        ttk.Label(header, text='Free Model Explorer', style='Header.TLabel').pack(anchor='w')
        ttk.Label(header, text='Live $0 catalogues · Kilo Gateway · OpenRouter · NVIDIA trial · OpenCode native', style='Sub.TLabel').pack(anchor='w')
        controls = ttk.Frame(self, padding=(20, 6, 20, 12)); controls.pack(fill='x')
        ttk.Label(controls, text='Search').pack(side='left')
        ttk.Entry(controls, width=38, textvariable=self.search).pack(side='left', padx=(8, 12))
        self.providers = ttk.Combobox(controls, state='readonly', textvariable=self.provider,
                                      values=['All providers'], width=19)
        self.providers.pack(side='left', padx=(0, 12))
        self.refresh_btn = ttk.Button(controls, text='↻  Refresh live', command=self._refresh)
        self.refresh_btn.pack(side='left')
        ttk.Button(controls, text='Export JSON', command=lambda: self._export('json')).pack(side='right')
        ttk.Button(controls, text='Export CSV', command=lambda: self._export('csv')).pack(side='right', padx=(0, 8))
        wrap = ttk.Frame(self, padding=(20, 0, 20, 0)); wrap.pack(fill='both', expand=True)
        columns = ('provider', 'name', 'id', 'context', 'route', 'evidence')
        self.tree = ttk.Treeview(wrap, columns=columns, show='headings', height=12)
        settings = {'provider': ('Provider', 120), 'name': ('Name', 215), 'id': ('Upstream model ID', 245),
                    'context': ('Context', 85), 'route': ('ZCode route ID', 255), 'evidence': ('Free evidence', 178)}
        for col, (label, width) in settings.items():
            self.tree.heading(col, text=label, command=lambda c=col: self._sort(c))
            self.tree.column(col, width=width, minwidth=70, anchor='w', stretch=True)
        self.tree.pack(side='left', fill='both', expand=True)
        sc = ttk.Scrollbar(wrap, orient='vertical', command=self.tree.yview)
        sc.pack(side='right', fill='y'); self.tree.configure(yscrollcommand=sc.set)
        self.tree.bind('<<TreeviewSelect>>', self._show_details)
        action = ttk.Frame(self, padding=(20, 12, 20, 8)); action.pack(fill='x')
        self.btn_upstream = ttk.Button(action, text='Copy upstream ID', command=lambda: self._copy('id'))
        self.btn_upstream.pack(side='left')
        self.btn_route = ttk.Button(action, text='Copy ZCode route ID', command=lambda: self._copy('route_id'))
        self.btn_route.pack(side='left', padx=(8, 0))
        ttk.Button(action, text='Copy ZCode base URL', command=lambda: self._copy_value(URL)).pack(side='left', padx=(8, 0))
        ttk.Button(action, text='Copy model details', command=self._copy_details).pack(side='left', padx=(8, 0))
        ttk.Label(action, text='Native-only OpenCode models cannot be routed through ZCode.', style='Sub.TLabel').pack(side='right')
        ttk.Label(self, text='Selected model details · exact API ID, free evidence, pricing, context and declared capabilities',
                  padding=(20, 6), style='Sub.TLabel').pack(anchor='w')
        details = ttk.Frame(self, padding=(20, 0, 20, 7)); details.pack(fill='both', expand=True)
        self.detail = tk.Text(details, height=10, bg='#111e2d', fg='#e5edf8', font=('Consolas', 10),
                              wrap='word', relief='flat', padx=14, pady=10, state='disabled',
                              selectbackground='#315e83')
        self.detail.pack(fill='both', expand=True)
        ttk.Label(self, textvariable=self.status, padding=(20, 8, 20, 13), style='Sub.TLabel').pack(fill='x')
        self._toggle_buttons()

    def _load_cache(self):
        cache = zfree.BASE / 'catalog.json'
        try:
            payload = json.loads(cache.read_text(encoding='utf-8'))
            self.models = normalize_models(payload.get('models', []))
            self._render()
            self.status.set('Cached data only — refresh to verify availability; cache is not live eligibility proof')
        except (OSError, ValueError, TypeError):
            self.status.set('No cached catalog · click Refresh live')

    def _refresh(self):
        self.refresh_btn.state(['disabled'])
        self.status.set('Checking public catalogs (no inference, no prompts sent)…')
        def worker():
            try:
                state = zfree.refresh(force=True)
                other, other_status = [], 'OpenCode not checked'
                try:
                    import os
                    other = opencode_native.inspect(os.getenv('ZFREE_OPENCODE_URL', 'http://127.0.0.1:4096'),
                                                   os.getenv('OPENCODE_SERVER_PASSWORD'))
                    other_status = f'OpenCode {len(other)} native-only'
                except (OSError, ValueError) as exc:
                    other_status = 'OpenCode local server unavailable'
                self.events.put(('catalog', normalize_models(state['models'], other),
                                 state['errors'], other_status))
            except Exception as exc:
                self.events.put(('error', type(exc).__name__ + ': ' + str(exc)[:140]))
        threading.Thread(target=worker, daemon=True).start()

    def _pump(self):
        try:
            while True:
                event = self.events.get_nowait()
                if event[0] == 'catalog':
                    _, self.models, errors, other_status = event
                    self._render()
                    self.status.set(f'{len(self.models)} catalog entries · {other_status} · '
                                    + ('Other catalog errors: ' + ', '.join(errors)[:170] if errors else 'catalog checks complete'))
                    self.refresh_btn.state(['!disabled'])
                else:
                    self.status.set('Refresh failed: ' + event[1])
                    self.refresh_btn.state(['!disabled'])
        except queue.Empty:
            pass
        self.after(100, self._pump)

    def _render(self):
        prior = self.selected.get('provider', '') + '/' + self.selected.get('id', '') if self.selected else ''
        providers = ['All providers'] + sorted({m.get('provider', '') for m in self.models})
        self.providers['values'] = providers
        if self.provider.get() not in providers:
            self.provider.set('All providers')
        self.visible = filtered(self.models, self.search.get(), self.provider.get())
        children = self.tree.get_children()
        if children:
            self.tree.delete(*children)
        focus = None
        for index, model in enumerate(self.visible):
            iid = str(index)
            self.tree.insert('', 'end', iid=iid, values=(model.get('provider', ''), model.get('name', ''),
                model['id'], f"{model.get('context_length') or 0:,}" if model.get('context_length') else '—',
                model.get('route_id') or 'Native only', model.get('evidence', '')))
            if model.get('provider', '') + '/' + model['id'] == prior:
                focus = iid
        if focus is not None:
            self.tree.selection_set(focus)
        else:
            self.selected = None
            self._detail_text('Select a model to inspect its details and copy the exact ID.')
            self._toggle_buttons()

    def _sort(self, col):
        # Stable sort within visible results; no hidden paid models are introduced.
        self.visible.sort(key=lambda m: as_text(m.get('route_id' if col == 'route' else
                                                   'context_length' if col == 'context' else col)).casefold())
        children = self.tree.get_children()
        if children:
            self.tree.delete(*children)
        for i, m in enumerate(self.visible):
            self.tree.insert('', 'end', iid=str(i), values=(m.get('provider', ''), m.get('name', ''), m['id'],
                f"{m.get('context_length') or 0:,}" if m.get('context_length') else '—',
                m.get('route_id') or 'Native only', m.get('evidence', '')))
        self.selected = None
        self._toggle_buttons()

    def _show_details(self, event=None):
        choices = self.tree.selection()
        if not choices:
            return
        index = int(choices[0])
        if index >= len(self.visible):
            return
        self.selected = self.visible[index]
        m = self.selected
        message = ('WARNING: NVIDIA is developer/trial only, not proven permanently free.\n\n'
                   if m['provider'] == 'nvidia-nim' else '')
        message += ('Model API ID: ' + m['id'] + '\n'
                    + 'ZCode route ID: ' + (m.get('route_id') or 'Not routable from ZCode') + '\n'
                    + 'Status: ' + as_text(m.get('evidence')) + '\n'
                    + 'Context: ' + as_text(m.get('context_length', 'unknown')) + '\n'
                    + 'Max output: ' + as_text(m.get('max_completion_tokens', 'unknown')) + '\n'
                    + 'Declared parameters: ' + ', '.join(m.get('supported_parameters', [])) + '\n'
                    + 'Input modalities: ' + ', '.join(m.get('input_modalities', [])) + '\n'
                    + 'Description: ' + as_text(m.get('description')) + '\n'
                    + 'Pricing: ' + json.dumps(m.get('pricing'), ensure_ascii=False) + '\n'
                    + 'Data/privacy note: ' + as_text(m.get('risk_note')) + '\n'
                    + 'Catalog URL: ' + as_text(m.get('catalog_source')))
        self._detail_text(message)
        self._toggle_buttons()

    def _detail_text(self, data):
        self.detail.configure(state='normal')
        self.detail.delete('1.0', 'end')
        self.detail.insert('1.0', data)
        self.detail.configure(state='disabled')

    def _toggle_buttons(self):
        self.btn_upstream.state(['!disabled'] if self.selected else ['disabled'])
        self.btn_route.state(['!disabled'] if self.selected and self.selected.get('route_id') else ['disabled'])

    def _copy_value(self, value):
        if value is None:
            return
        self.clipboard_clear(); self.clipboard_append(value); self.update_idletasks()
        self.status.set('Copied to clipboard')

    def _copy(self, key):
        if self.selected:
            self._copy_value(self.selected.get(key))

    def _copy_details(self):
        if self.selected:
            self._copy_value(json.dumps(self.selected, ensure_ascii=False, indent=2))

    def _export(self, ext):
        if not self.visible:
            messagebox.showinfo('Nothing to export', 'Refresh or change filters to show models.')
            return
        filename = filedialog.asksaveasfilename(defaultextension='.' + ext, filetypes=[(ext.upper(), '*.' + ext)],
                                                 initialfile='zfree-models.' + ext)
        if not filename:
            return
        try:
            if ext == 'json':
                data = json.dumps({'free_only': True, 'models': self.visible}, indent=2, ensure_ascii=False)
            else:
                output = io.StringIO(); writer = csv.writer(output)
                writer.writerow(('provider', 'upstream_id', 'zcode_route_id', 'name', 'context', 'evidence', 'tools_declared'))
                for m in self.visible:
                    writer.writerow((m.get('provider'), m['id'], m.get('route_id'), m.get('name'),
                                     m.get('context_length'), m.get('evidence'),
                                     'tools' in m.get('supported_parameters', [])))
                data = output.getvalue()
            pathlib.Path(filename).write_text(data, encoding='utf-8')
            self.status.set('Export saved: ' + filename)
        except OSError as exc:
            messagebox.showerror('Export failed', str(exc))


def main():
    Explorer().mainloop()

if __name__ == '__main__':
    main()