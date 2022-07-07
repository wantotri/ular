(()=>{"use strict";var e,n,t,r,o={10:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(84),o=e([r]);async function c(){let e=document.getElementById("snake-app"),n=document.getElementById("score");e.innerText=r.fK(),document.getElementById("snake-play-btn").addEventListener("click",(t=>{let o=r.j9(),c=r.j9();document.getElementById("snake-key-left").addEventListener("click",(()=>{3!=o&&(c=1)})),document.getElementById("snake-key-up").addEventListener("click",(()=>{0!=o&&(c=2)})),document.getElementById("snake-key-right").addEventListener("click",(()=>{1!=o&&(c=3)})),document.getElementById("snake-key-down").addEventListener("click",(()=>{2!=o&&(c=0)})),document.onkeydown=e=>{switch((e=e||window.event).key||e.keyCode){case 37:case"ArrowLeft":e.preventDefault(),3!=o&&(c=1);break;case 38:case"ArrowUp":e.preventDefault(),0!=o&&(c=2);break;case 39:case"ArrowRight":e.preventDefault(),1!=o&&(c=3);break;case 40:case"ArrowDown":e.preventDefault(),2!=o&&(c=0)}};const a=setInterval((()=>{r.Lu(c),r.CB(),e.innerText=r.fK(),n.innerText=r.vi(),o=r.j9(),r.eR()&&(n.innerText+="   ⛔ Game Over ⛔",clearInterval(a))}),300)}))}r=(o.then?(await o)():o)[0],c(),n()}catch(a){n(a)}}))},84:(e,n,t)=>{t.a(e,(async(r,o)=>{try{t.d(n,{$r:()=>X,CB:()=>k,CU:()=>F,DX:()=>$,Lu:()=>v,Nu:()=>Y,Or:()=>J,QZ:()=>C,Rh:()=>K,SH:()=>I,VU:()=>q,Vz:()=>V,Wl:()=>O,XP:()=>N,Z5:()=>P,_d:()=>T,aB:()=>M,cA:()=>D,dx:()=>G,eR:()=>E,eY:()=>U,en:()=>H,fK:()=>m,hF:()=>R,j9:()=>h,m_:()=>W,oH:()=>ee,uV:()=>Q,ug:()=>B,vi:()=>x,wF:()=>A,xR:()=>z,xe:()=>Z,z1:()=>L});var c=t(429);e=t.hmd(e);var a=r([c]);c=(a.then?(await a)():a)[0];const i=new Array(32).fill(void 0);function u(e){return i[e]}i.push(void 0,null,!0,!1);let _=i.length;function d(e){e<36||(i[e]=_,_=e)}function f(e){const n=u(e);return d(e),n}function s(e){_===i.length&&i.push(i.length+1);const n=_;return _=i[n],i[n]=e,n}let b,l,w=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function g(){return 0===b.byteLength&&(b=new Uint8Array(c.memory.buffer)),b}function y(e,n){return w.decode(g().subarray(e,e+n))}function p(){return 0===l.byteLength&&(l=new Int32Array(c.memory.buffer)),l}function m(){try{const t=c.__wbindgen_add_to_stack_pointer(-16);c.gamePrint(t);var e=p()[t/4+0],n=p()[t/4+1];return y(e,n)}finally{c.__wbindgen_add_to_stack_pointer(16),c.__wbindgen_free(e,n)}}function h(){return c.getSnakeDirection()}function v(e){c.setSnakeDirection(e)}function k(){c.gameTick()}function x(){return c.gameScore()>>>0}function E(){return 0!==c.gameIsOver()}function S(e,n){try{return e.apply(this,n)}catch(e){c.__wbindgen_exn_store(s(e))}}function j(e,n){return g().subarray(e/1,e/1+n)}function A(){return S((function(e,n,t){u(e).randomFillSync(j(n,t))}),arguments)}function B(e){f(e)}function D(){return S((function(e,n){u(e).getRandomValues(u(n))}),arguments)}function I(e){return s(u(e).process)}function O(e){const n=u(e);return"object"==typeof n&&null!==n}function T(e){return s(u(e).versions)}function L(e){return s(u(e).node)}function U(e){return"string"==typeof u(e)}function P(){return s(e)}function R(){return S((function(e,n,t){return s(u(e).require(y(n,t)))}),arguments)}function F(e){return s(u(e).crypto)}function V(e){return s(u(e).msCrypto)}function C(e,n){return s(new Function(y(e,n)))}function q(){return S((function(e,n){return s(u(e).call(u(n)))}),arguments)}function W(e){return s(u(e))}function $(){return S((function(){return s(self.self)}),arguments)}function z(){return S((function(){return s(window.window)}),arguments)}function H(){return S((function(){return s(globalThis.globalThis)}),arguments)}function M(){return S((function(){return s(t.g.global)}),arguments)}function N(e){return void 0===u(e)}function X(e){return s(u(e).buffer)}function Z(e){return s(new Uint8Array(u(e)))}function K(e,n,t){u(e).set(u(n),t>>>0)}function Q(e){return u(e).length}function Y(e){return s(new Uint8Array(e>>>0))}function G(e,n,t){return s(u(e).subarray(n>>>0,t>>>0))}function J(e,n){throw new Error(y(e,n))}function ee(){return s(c.memory)}w.decode(),l=new Int32Array(c.memory.buffer),b=new Uint8Array(c.memory.buffer),o()}catch(ne){o(ne)}}))},429:(e,n,t)=>{t.a(e,(async(r,o)=>{try{var c,a=r([c=t(84)]),[c]=a.then?(await a)():a;await t.v(n,e.id,"7a881a441a85ca0b62fc",{"./ular_bg.js":{__wbg_randomFillSync_91e2b39becca6147:c.wF,__wbindgen_object_drop_ref:c.ug,__wbg_getRandomValues_b14734aa289bc356:c.cA,__wbg_process_e56fd54cf6319b6c:c.SH,__wbindgen_is_object:c.Wl,__wbg_versions_77e21455908dad33:c._d,__wbg_node_0dd25d832e4785d5:c.z1,__wbindgen_is_string:c.eY,__wbg_static_accessor_NODE_MODULE_26b231378c1be7dd:c.Z5,__wbg_require_0db1598d9ccecb30:c.hF,__wbg_crypto_b95d7173266618a9:c.CU,__wbg_msCrypto_5a86d77a66230f81:c.Vz,__wbg_newnoargs_fc5356289219b93b:c.QZ,__wbg_call_4573f605ca4b5f10:c.VU,__wbindgen_object_clone_ref:c.m_,__wbg_self_ba1ddafe9ea7a3a2:c.DX,__wbg_window_be3cc430364fd32c:c.xR,__wbg_globalThis_56d9c9f814daeeee:c.en,__wbg_global_8c35aeee4ac77f2b:c.aB,__wbindgen_is_undefined:c.XP,__wbg_buffer_de1150f91b23aa89:c.$r,__wbg_new_97cf52648830a70d:c.xe,__wbg_set_a0172b213e2469e9:c.Rh,__wbg_length_e09c0b925ab8de5d:c.uV,__wbg_newwithlength_e833b89f9db02732:c.Nu,__wbg_subarray_9482ae5cd5cd99d3:c.dx,__wbindgen_throw:c.Or,__wbindgen_memory:c.oH}}),o()}catch(e){o(e)}}),1)}},c={};function a(e){var n=c[e];if(void 0!==n)return n.exports;var t=c[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,a),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&!e.d&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},a.a=(o,c,a)=>{var i;a&&((i=[]).d=1),i&&(i.moduleId=o.id);var u,_,d,f=new Set,s=o.exports,b=new Promise(((e,n)=>{d=n,_=e}));b[n]=s,b[e]=e=>(i&&e(i),f.forEach(e),b.catch((e=>{}))),b.moduleId=o.id,o.exports=b,c((o=>{var c;u=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var c=[];c.d=0,o.then((e=>{a[n]=e,r(c)}),(e=>{a[t]=e,r(c)}));var a={};return a[e]=e=>e(c),a}}var i={};return i[e]=e=>{},i[n]=o,i})))(o);var a=()=>u.map((e=>{if(e[t])throw e[t];return e[n]})),_=new Promise((n=>{(c=()=>n(a)).r=0;var t=e=>e!==i&&!f.has(e)&&(f.add(e),e&&!e.d&&(c.r++,e.push(c)));u.map((n=>n[e](t)))}));return c.r?_:a()}),(e=>(e?d(b[t]=e):_(s),r(i)))),i&&(i.d=0)},a.d=(e,n)=>{for(var t in n)a.o(n,t)&&!a.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},a.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),a.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),a.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),a.v=(e,n,t,r)=>{var o=fetch(a.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((n=>Object.assign(e,n.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},(()=>{var e;a.g.importScripts&&(e=a.g.location+"");var n=a.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");t.length&&(e=t[t.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),a.p=e})(),a(10)})();