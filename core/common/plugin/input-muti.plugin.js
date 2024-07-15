var InputMutiPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var isDisabled = tmpObj.hasClass("disabled");
			
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			var width = tmpObj.attr("width");
			
			tmpObj.css("width",width);
			
			var placeholder = tmpObj.attr("placeholder");
			
			var height = tmpObj.attr("height");
			if(height==null)height="102px";
			
			var maxlength = tmpObj.attr("maxlength");
			var wrappedLabel = tmpObj.attr("label");//标签自带的label
			if(maxlength==null)maxlength=30;
			
			var replaceHtml = null;
			if(wrappedLabel==null){
				replaceHtml = '<div class="input-muti-wrapper region-wrapper">';
			}
			else{
				replaceHtml = '<div class="input-muti-wrapper region-wrapper"><span class="wrapped-label">'+wrappedLabel+'</span>';
			}
			if(isDisabled)
				replaceHtml+='<textarea region-attr="'+tmpObj.attr("region-attr")+'" class="wrapped input-area form-control disabled" style="height:'+height+'" ></textarea>';
			else 
				replaceHtml+='<textarea region-attr="'+tmpObj.attr("region-attr")+'" class="region-editable wrapped input-area form-control" style="height:'+height+'" ></textarea>';
			replaceHtml += '	<div class="text-count-wrap"><div class="text-count hidden"></div></div>';
			if(placeholder!=null)replaceHtml += '	<div class="place-holder hidden">'+placeholder+'</div>';
			else replaceHtml += '	<div class="place-holder hidden"></div>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			
			if(width!=null){
				replaced.css("width",width);
			}
			
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
			
			replaced.find(".input-area").focus(function(){
				replaced.find(".text-count").removeClass("hidden");
			});
			replaced.find(".input-area").blur(function(){
				replaced.find(".text-count").addClass("hidden");
			});
			
			replaced.find(".input-area").val(paraData);
			try{
				paraData = JSON.stringify(JSON.parse(paraData), null, 2);
			}catch(e){}
			
			replaced.find(".input-area").val(paraData);//replaced.find(".input-area")[0].innerText=paraData;
			
			setTimeout(function(){
				//replaced.find(".text-count").css("right",50-replaced.outerWidth(true));
				replaced.find(".place-holder").css("top","8px");
			});
			if(paraData.length==0){
				replaced.find(".place-holder").removeClass("hidden");
			}
			
			
			replaced.find(".input-area").on("input",function(){
				var curText = $(this).val();
				var available = maxlength-(curText+"").length;
				var wrapped = replaced.find(".wrapped");
				if(available<0){
					$(this).val((curText+"").substring(0,maxlength));
					//让DIV光标跳到末尾
					/*var range = document.createRange();
			        range.selectNodeContents(this);
			        range.collapse(false);
			        var sel = window.getSelection();
			        sel.removeAllRanges();
			        sel.addRange(range);*/
			        replaced.find(".text-count").text(0);
				}
				else {
					replaced.find(".text-count").text(available);
					//wrapped.val(curText);
				}
				
				
				
				if(curText.length>0)
					replaced.find(".place-holder").addClass("hidden");
				else
					replaced.find(".place-holder").removeClass("hidden");
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(wrapped,curText)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			if(paraData==null)paraData="";
			
			try{
				paraData = JSON.stringify(JSON.parse(paraData), null, 2);
			}catch(e){}
			
			regionElemObj.val(paraData);
			var maxlength = regionElemObj.attr("maxlength");
			if(maxlength==null)maxlength=30;
			var available = maxlength-paraData.length;
			wrapperElemObj.find(".text-count").text(available);
			wrapperElemObj.find(".input-area").val(paraData);
			
			if(paraData.length>0)
				wrapperElemObj.find(".place-holder").addClass("hidden");
			else
				wrapperElemObj.find(".place-holder").removeClass("hidden");
		}
}