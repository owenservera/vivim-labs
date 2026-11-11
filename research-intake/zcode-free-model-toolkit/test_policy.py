"""Offline ZFree policy regression suite. Does not call external providers."""
import pathlib
import tempfile
import unittest
from policy import strict_zero_price, demand, select, prepare, Quota, StickyRoutes, session_key, has_tool_history, nim_chat_candidate

FREE = dict(id="n/model:free", provider="openrouter", supported_parameters=["tools", "reasoning", "tool_choice"], input_modalities=["text"], max_completion_tokens=2048)

class PolicyTests(unittest.TestCase):
    def test_strict_zero(self):
        self.assertTrue(strict_zero_price({"prompt":"0", "completion":"0.0", "request":"0"}))
        for p in ({"prompt":"0"}, {"prompt":"0","completion":"0","request":"0.5"}, {"prompt":"0","completion":None}, {"prompt":"nan","completion":"0"}, {}):
            self.assertFalse(strict_zero_price(p))

    def test_guards(self):
        body={"messages":[{"role":"user","content":"x"}], "plugins":[{"id":"web"}]}
        with self.assertRaisesRegex(ValueError,"Paid"):
            demand(body)
        body={"messages":[{"role":"user","content":"x"}], "tools":[{"type":"openrouter:web_search"}]}
        with self.assertRaisesRegex(ValueError,"Only"):
            demand(body)

    def test_tools(self):
        b={"messages":[{"role":"user","content":"x"}],"tools":[{"type":"function","function":{"name":"go"}}]}
        self.assertEqual(select([FREE], b), [FREE])
        self.assertEqual(select([dict(FREE, supported_parameters=[])], b), [])
        self.assertEqual(select([dict(FREE, provider="nvidia-nim")], b), [])
        self.assertEqual(select([dict(FREE, provider="nvidia-nim")], b, nim_tool_ids=[FREE["id"]])[0]["id"], FREE["id"])

    def test_tool_result_requires_tool_capable_model(self):
        b = {"messages": [
            {"role": "user", "content": "run"},
            {"role": "tool", "tool_call_id": "id", "content": "{}"}]}
        self.assertEqual(select([dict(FREE, supported_parameters=[])], b), [])
        self.assertEqual(select([FREE], b), [FREE])

    def test_max_token_field_mapping(self):
        req = {"messages": [{"role": "user", "content": "x"}], "max_completion_tokens": 8000}
        legacy = dict(FREE, supported_parameters=["tools", "max_tokens"])
        sent = prepare(req, legacy)
        self.assertEqual(sent["max_tokens"], 2048)
        self.assertNotIn("max_completion_tokens", sent)
        modern = dict(FREE, supported_parameters=["max_completion_tokens"])
        sent = prepare(req, modern)
        self.assertEqual(sent["max_completion_tokens"], 2048)
        self.assertNotIn("max_tokens", sent)

    def test_images(self):
        b={"messages":[{"role":"user","content":[{"type":"image_url","image_url":{"url":"x"}}]}]}
        self.assertEqual(select([FREE], b), [])
        self.assertEqual(select([dict(FREE, input_modalities=["text","image"])], b)[0]["id"], FREE["id"])

    def test_nim_filter(self):
        self.assertFalse(nim_chat_candidate("nvidia/embed-qa"))
        self.assertFalse(nim_chat_candidate("nvidia/nemoguard"))
        self.assertTrue(nim_chat_candidate("meta/llama-3.3-70b-instruct"))

    def test_reasoning_preserves_tool_history(self):
        b={"messages":[{"role":"tool","tool_call_id":"a","content":"{}"}], "reasoning":{"effort":"high"}, "reasoning_effort":"medium", "thinking":True,"max_completion_tokens":99999}
        a=prepare(b,FREE)
        self.assertEqual(a["reasoning"], {"effort":"medium"})
        self.assertNotIn("reasoning_effort",a)
        self.assertNotIn("thinking",a)
        self.assertEqual(a["messages"],b["messages"])
        self.assertEqual(a["max_tokens"],2048)

    def test_daily_quota_survives_restart(self):
        with tempfile.TemporaryDirectory() as d:
            p=pathlib.Path(d)/"quota.json"
            q=Quota(p, rpm=20, daily=2, clock=lambda:1000)
            self.assertTrue(q.take()[0])
            self.assertTrue(q.take()[0])
            self.assertFalse(q.take()[0])
            q2=Quota(p, rpm=20, daily=2, clock=lambda:1001)
            self.assertFalse(q2.take()[0])

    def test_minute_quota(self):
        with tempfile.TemporaryDirectory() as d:
            q=Quota(pathlib.Path(d)/"quota.json",rpm=1, daily=50, clock=lambda:1000)
            self.assertTrue(q.take()[0])
            self.assertFalse(q.take()[0])

    def test_stickiness(self):
        q={"messages":[{"role":"system","content":"sys"},{"role":"user","content":"hello"}]}
        k=session_key(q)
        self.assertEqual(session_key(q),k)
        self.assertFalse(has_tool_history(q))
        s=StickyRoutes(ttl=2,clock=lambda:100)
        s.put(k,"model:free")
        self.assertEqual(s.get(k),"model:free")
        self.assertTrue(has_tool_history({"messages":[{"role":"tool","content":"result"}]}))
        self.assertNotEqual(session_key(q,explicit="id-2"),k)

if __name__ == "__main__":
    unittest.main()
