(function() {var implementors = {};
implementors["packed_simd_2"] = [{"text":"impl&lt;A&gt; Send for Simd&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as SimdArray&gt;::Tuple: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for LexicographicallyOrdered&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for m8","synthetic":true,"types":[]},{"text":"impl Send for m16","synthetic":true,"types":[]},{"text":"impl Send for m32","synthetic":true,"types":[]},{"text":"impl Send for m64","synthetic":true,"types":[]},{"text":"impl Send for m128","synthetic":true,"types":[]},{"text":"impl Send for msize","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()