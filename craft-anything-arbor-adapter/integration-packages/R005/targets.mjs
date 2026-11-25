// Research definitions, not provider registrations. Pinned ARBOR source.
export const ARBOR_PIN='cc4f001fe835b1ceaa20e8cb45fa0353081f41ab';
const chat=['navigation','conversation.list','conversation.search','conversation.open','conversation.create','composer','attachment','model.select','reply.read','reply.stop','message.actions','settings','account.privacy'];
const specs=[
 ['chatgpt','VIV-0001','https://chatgpt.com','ai-chat',['projects','memory','tools'],['34f91b8c475f85210d13adc56545c8eb1526d2e6','22e74c737c8892236d73eade409b0cc8d7e6591f']],
 ['gemini','VIV-0025','https://gemini.google.com','ai-chat',['gems','activity','tools','connected-apps'],['e7fea2cdc92341220c4ee7901cf859b82ed9885a','e528ac6fb55aa953e5712dd1d88ec1cb1628c020']],
 ['claude','VIV-0002','https://claude.ai','ai-chat',['projects','artifacts','preferences'],['28cc85bf1f5e03a029525cae5c60240f27e5a75b','b67048aa3501028d8b77795f8c41217fc3c3d743']],
 ['facebook','VIV-0016','https://www.facebook.com','social',['navigation','feed','post.read','post.compose','comment','reaction','share','profile','search','groups','pages','notifications','messages','media','settings','account.privacy'],[]]
];
export const TARGETS=Object.freeze(Object.fromEntries(specs.map(([siteId,targetId,origin,kind,extra,blobs])=>[siteId,Object.freeze({
 siteId,targetId,origin,kind,surfaces:Object.freeze(kind==='ai-chat'?[...chat,...extra]:extra),
 sources:Object.freeze(blobs.map((blobSha,i)=>Object.freeze({owner:i?'harness':'studio',blobSha,path:i?'harness/src/providers/'+siteId+'.mjs':'studio/src/profiles/sites/'+siteId+'.mjs'})))
})])));
export const OPERATIONS=Object.freeze({
 'conversation.list':{surface:'conversation.list',effect:'read',input:['cursor'],output:'conversation-page'},
 'conversation.search':{surface:'conversation.search',effect:'read',input:['query','cursor'],output:'conversation-page'},
 'conversation.open':{surface:'conversation.open',effect:'navigation',input:['conversationRef'],output:'conversation-view'},
 'conversation.create':{surface:'conversation.create',effect:'write',input:[],output:'conversation-ref'},
 'message.send':{surface:'composer',effect:'write',input:['conversationRef','contentRef'],output:'message-receipt'},
 'reply.read':{surface:'reply.read',effect:'read',input:['conversationRef'],output:'reply-view'},
 'settings.read':{surface:'settings',effect:'read',input:[],output:'settings-tree'},
 'feed.read':{surface:'feed',effect:'read',input:['cursor'],output:'post-page'},
 'post.read':{surface:'post.read',effect:'read',input:['postRef'],output:'post-view'},
 'post.create':{surface:'post.compose',effect:'write',input:['contentRef'],output:'post-receipt'},
 'comment.create':{surface:'comment',effect:'write',input:['postRef','contentRef'],output:'comment-receipt'},
 'social.search':{surface:'search',effect:'read',input:['query','cursor'],output:'search-page'}
});
// IDs above are proposal keys, not canonical HQ semantic IDs or callable actions.
