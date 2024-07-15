var RadioPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var isDisabled = tmpObj.hasClass("disabled");
			
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			
			var width = tmpObj.attr("width");
			var layout = tmpObj.attr("layout");
			if(layout==null)
				layout="h";//默认horizontal   可选值h,v
			
			
			var regionDsName = tmpObj.attr("region-ds");
			var options = RegionUtil.getRegionDsList(region,regionDsName);

			var hiddenInputHtml = tmpObj.prop("outerHTML");;
			
			var replaceHtml = '<div class="radio-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml+='<div class="options">';
			if(options!=null){
				for(var i = 0 ; i <options.length ; i++){
					if(layout=="h"){
						replaceHtml+='<div class="radio-item">';
					}
					else{
						replaceHtml+='<div class="radio-item vertical">';
					}
					
					
					if(paraData==options[i].value)
						replaceHtml+='	<div class="radio-input checked"></div>';
					else
						replaceHtml+='	<div class="radio-input"></div>';
					replaceHtml+='	<div class="text" val='+options[i].value+'>'+options[i].text+'</div>';
					replaceHtml+='</div>';
				}	
			}
			replaceHtml+='	</div>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			if(isDisabled){
				replaced.addClass("disabled");
			}
			
			replaced.find(".radio-item").click(function(){
				var radioItem = $(this);
				var inputItem = radioItem.find(".radio-input");
				var checked = inputItem.hasClass("checked");
				var newVal = null;
				if(!checked){
					radioItem.parent().find(".radio-input").removeClass("checked");
					inputItem.addClass("checked");
					newVal = radioItem.find(".text").attr("val");
					replaced.find(".wrapped").val(newVal);
					
					if(onchange!=null&&onchange!=""){
						eval(onchange+'.call(this,newVal)');
						//eval(onchange+"('"+newVal+"')");
					}
				}
				
				
			});
			
			tmpObj = replaced.find(".wrapped");
			if(width!=null){
				replaced.css("width",width);
			}
			
			tmpObj.val(paraData);
			return tmpObj;
		},
		refreshDataSource:function(region,wrapperElemObj,regionElemObj,options){
			var paraData = regionElemObj.val();
			var onchange = regionElemObj.attr("onchange");
			
			var layout = regionElemObj.attr("layout");
			if(layout==null)
				layout="h";//默认horizontal   可选值h,v
			
			var optionsHtml = "";
			if(options!=null){
				for(var i = 0 ; i <options.length ; i++){
					if(layout=="h"){
						optionsHtml+='<div class="radio-item">';
					}
					else{
						optionsHtml+='<div class="radio-item vertical">';
					}
					
					if(paraData==options[i].value)
						optionsHtml+='	<div class="radio-input checked"></div>';
					else
						optionsHtml+='	<div class="radio-input"></div>';
					optionsHtml+='	<div class="text" val='+options[i].value+'>'+options[i].text+'</div>';
					optionsHtml+='</div>';
				}	
			}
			wrapperElemObj.find(".options").html(optionsHtml);
			RegionUtil.addRegionUniqueId(wrapperElemObj[0],region.regionId);
			
			wrapperElemObj.find(".radio-item").click(function(){
				var radioItem = $(this);
				var inputItem = radioItem.find(".radio-input");
				var checked = inputItem.hasClass("checked");
				var newVal = null;
				if(!checked){
					radioItem.parent().find(".radio-input").removeClass("checked");
					inputItem.addClass("checked");
					newVal = radioItem.find(".text").attr("val");
					wrapperElemObj.find(".wrapped").val(newVal);
				}
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(this,newVal)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);

			wrapperElemObj.find(".radio-input").removeClass("checked");
			wrapperElemObj.find(".radio-item").each(function(){
				var radioItem = $(this);
				var inputItem = radioItem.find(".radio-input");
				if(paraData == radioItem.find(".text").attr("val")){
					inputItem.addClass("checked");
					return false;//结束循环
				}
			});
		}
}