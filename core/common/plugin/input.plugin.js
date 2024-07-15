var InputPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var isDisabled = tmpObj.hasClass("disabled");
			if(isDisabled){
				tmpObj.attr("readonly","readonly");
			}
			
			var readonly = tmpObj.attr("readonly");
			if(readonly!=undefined){
				isDisabled = true;
			}

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
				replaceHtml = '<div class="input-wrapper region-wrapper">'+tmpObj.prop("outerHTML");
			}
			else{
				replaceHtml = '<div class="input-wrapper region-wrapper"><span class="wrapped-label">'+wrappedLabel+'</span>'+tmpObj.prop("outerHTML");
			}
			
			var showCount = tmpObj.attr("show-count");
			var quickClear = "";
			if(!isDisabled){
				quickClear = '<div class="quick-clear hidden"><i class="fa-light fa-close"></i></div>';
			}
			if(showCount=="true"){
				if(tail!=null)
					replaceHtml += '	<div class="text-count-wrap"><div class="text-count hidden text-count-ref"></div> '+quickClear+'  <div class="text-tail"></div></div>';
				else
					replaceHtml += '	<div class="text-count-wrap"><div class="text-count hidden text-count-ref"></div> '+quickClear+'  </div>';
			}
			else{
				if(tail!=null)
					replaceHtml += '	<div class="text-count-wrap">'+quickClear+'<div class="text-tail"></div></div>';
				else
					replaceHtml += '	<div class="text-count-wrap">'+quickClear+'</div>';
			}
			
			
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			
			if(width!=null){
				replaced.css("width",width);
			}
			if(tail!=null)replaced.find(".text-tail").html(tail);
			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			tmpObj = replaced.find(".wrapped");
			
			var count = maxlength;
			
			if(paraData==null||paraData==""){
				paraData = tmpObj.attr("default");
			}
			
			if(paraData!=null){
				paraData=paraData+"";
				count-=paraData.length;
			}
			else{
				paraData="";
			}
			replaced.find(".text-count").text(count);
			
			tmpObj.val(paraData);
			
			tmpObj.focus(function(){
				replaced.find(".text-count").removeClass("hidden");
				replaced.find(".quick-clear").removeClass("hidden");
			});
			tmpObj.blur(function(){
				replaced.find(".text-count").addClass("hidden");
				setTimeout(function(){
					replaced.find(".quick-clear").addClass("hidden");
				},200)
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

			replaced.find(".quick-clear>.fa-close").click(function(){
				$(this).closest(".region-wrapper").find(".wrapped").val("");
			})

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
			if(paraData==null||paraData==""){
				paraData = regionElemObj.attr("default");
			}
			
			if(paraData==null)paraData="";

			regionElemObj.val(paraData);
			var maxlength = regionElemObj.attr("maxlength");
			if(maxlength==null)maxlength=30;
			var available = maxlength-paraData.length;
			wrapperElemObj.find(".text-count").text(available);
		}
}