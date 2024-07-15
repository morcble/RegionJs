var PasswordPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var hide = tmpObj.attr("hide");
			if("true"==hide){
				//tmpObj.attr("type","hidden");
				tmpObj.closest(".data-block-wrapper").addClass("hidden");
			}
			
			var tail = tmpObj.attr("tail");
			
			tmpObj.addClass("wrapped");
			var width = tmpObj.attr("width");
			if(width!=null){
				tmpObj.css("width",width);
			}
			
			var maxlength = tmpObj.attr("maxlength");
			var wrappedLabel = tmpObj.attr("label");//标签自带的label
			if(maxlength==null)maxlength=30;
			
			var replaceHtml = null;
			if(wrappedLabel==null){
				replaceHtml = '<div class="password-wrapper region-wrapper">'+tmpObj.prop("outerHTML");
			}
			else{
				replaceHtml = '<div class="password-wrapper region-wrapper"><span class="wrapped-label">'+wrappedLabel+'</span>'+tmpObj.prop("outerHTML");
			}
			
			if(tail!=null)
				replaceHtml += '	<div class="text-count-wrap"><div class="text-tail"></div><div class="text-count text-count-ref hidden"></div></div>';
			else
				replaceHtml += '	<div class="text-count-wrap"><div class="text-count text-count-ref hidden"></div></div>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			
			if(width!=null){
				replaced.css("width",width);
			}
			if(tail!=null)replaced.find(".text-tail").html(tail);
			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			var count = maxlength;
			if(paraData!=null){
				paraData=paraData+"";
				count-=paraData.length;
			}
			else{
				paraData="";
			}
			replaced.find(".text-count").text(count);
			
			tmpObj = replaced.find(".wrapped");
			tmpObj.val(paraData);
			
			tmpObj.focus(function(){
				replaced.find(".text-count").removeClass("hidden");
			});
			tmpObj.blur(function(){
				replaced.find(".text-count").addClass("hidden");
			});
			
			tmpObj.on("input",function(){
				var available = maxlength-($(this).val()+"").length;
				var wrapped = replaced.find(".wrapped");
				var  curText= wrapped.val()
				if(available<0){
					wrapped.val((curText+"").substring(0,maxlength));
				}
				else replaced.find(".text-count").text(available);
				
				if(onchange!=null&&onchange!=""){
					var indexTag = onchange.indexOf("(");
					if(indexTag==-1){
						eval(onchange+'.call(wrapped,curText)');
					}
					else{
						var script = onchange.substring(0,indexTag)+".call(wrapped,curText,"+onchange.substring(indexTag+1,onchange.length);
						eval(script)
					}
				}
			});
			
			var onsearch = tmpObj.attr("onsearch");
			if(onsearch!=null&&onsearch.trim()!=""){
				tmpObj.on("focus",function(){
					var wrapped = replaced.find(".wrapped");
					var  curText= wrapped.val()
					var indexTag = onsearch.indexOf("(");
					if(indexTag==-1){
						eval(onsearch+'.call(wrapped,curText)');
					}
					else{
						var script = onsearch.substring(0,indexTag)+".call(wrapped,curText,"+onsearch.substring(indexTag+1,onsearch.length);
						eval(script)
					}
				});
				tmpObj.on("input",function(){
					var wrapped = replaced.find(".wrapped");
					var  curText= wrapped.val()
					var indexTag = onsearch.indexOf("(");
					if(indexTag==-1){
						eval(onsearch+'.call(wrapped,curText)');
					}
					else{
						var script = onsearch.substring(0,indexTag)+".call(wrapped,curText,"+onsearch.substring(indexTag+1,onsearch.length);
						eval(script)
					}
				});
			}
			
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			if(paraData==null)paraData="";
			regionElemObj.val(paraData);
			var maxlength = regionElemObj.attr("maxlength");
			if(maxlength==null)maxlength=30;
			var available = maxlength-paraData.length;
			wrapperElemObj.find(".text-count").text(available);
		}
}