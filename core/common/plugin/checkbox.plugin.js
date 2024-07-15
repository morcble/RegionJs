var CheckboxPlugin = {
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

			var hiddenInputHtml = tmpObj.prop("outerHTML");
			
			if(paraData==null)paraData="";
			else paraData+="";
			if(paraData.endWith(",")){
				paraData=paraData.substring(0,paraData.length-1);
			}
			var currentValueArray = paraData.split(",");
			var valueCache = {};
			for(var i=0;i<currentValueArray.length;i++){
				valueCache[currentValueArray[i]] = true;
			}
			
			var replaceHtml = '<div class="checkbox-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml+='<div class="options">';
			if(options!=null){
				for(var i = 0 ; i <options.length ; i++){
					if(layout=="h"){
						replaceHtml+='<div class="checkbox-item">';
					}
					else{
						replaceHtml+='<div class="checkbox-item vertical">';
					}
					
					
					if(valueCache[options[i].value])
						replaceHtml+='	<div class="checkbox-input checked"></div>';
					else
						replaceHtml+='	<div class="checkbox-input"></div>';
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
			
			replaced.find(".checkbox-item").click(function(){
				var radioItem = $(this);
				var inputItem = radioItem.find(".checkbox-input");
				var checked = inputItem.hasClass("checked");
				var newVal = null;
				if(!checked){
					inputItem.addClass("checked");
				}
				else{
					inputItem.removeClass("checked");
				}
				
				replaced.find(".checked").each(function(){
					if(newVal==null||newVal=="")newVal=$(this).parent().find(".text").attr("val");
					else newVal+=","+$(this).parent().find(".text").attr("val");
				});
				
				replaced.find(".wrapped").val(newVal);
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(this,newVal)');
					//eval(onchange+"('"+newVal+"')");
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
			var optionsHtml = "";
			if(options!=null){
				for(var i = 0 ; i <options.length ; i++){
					optionsHtml+='<div class="checkbox-item">';
					
					if(paraData==options[i].value)
						optionsHtml+='	<div class="checkbox-input checked"></div>';
					else
						optionsHtml+='	<div class="checkbox-input"></div>';
					optionsHtml+='	<div class="text" val='+options[i].value+'>'+options[i].text+'</div>';
					optionsHtml+='</div>';
				}	
			}
			wrapperElemObj.find(".options").html(optionsHtml);
			RegionUtil.addRegionUniqueId(wrapperElemObj[0],region.regionId);
			
			wrapperElemObj.find(".checkbox-item").click(function(){
				var radioItem = $(this);
				var inputItem = radioItem.find(".checkbox-input");
				var checked = inputItem.hasClass("checked");
				var newVal = null;
				if(!checked){
					inputItem.addClass("checked");
				}
				else{
					inputItem.removeClass("checked");
				}
				
				wrapperElemObj.find(".checked").each(function(){
					if(newVal==null||newVal=="")newVal=$(this).parent().find(".text").attr("val");
					else newVal+=","+$(this).parent().find(".text").attr("val");
				});
				
				wrapperElemObj.find(".wrapped").val(newVal);
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(this,newVal)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
			
			//重新初始化
			if(paraData.endWith(",")){
				paraData=paraData.substring(0,paraData.length-1);
			}
			var currentValueArray = paraData.split(",");
			var valueCache = {};
			for(var i=0;i<currentValueArray.length;i++){
				valueCache[currentValueArray[i]] = true;
			}
			
			var newVal = "";
			var initCheckItems = wrapperElemObj.find(".checkbox-input");
			initCheckItems.each(function(){
				var textObj = $(this).parent().children(".text");
				var value = textObj.attr("val");
				if(valueCache[value]){
					$(this).addClass("checked");
					if(newVal==null)newVal=textObj.attr("val");
					else newVal+=","+textObj.attr("val");
				}
				else{
					$(this).removeClass("checked");
				}
			});
			wrapperElemObj.find(".wrapped").val(newVal);
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);

			if(paraData==null)paraData="";
			else paraData=""+paraData;
			if(paraData.endWith(",")){
				paraData=paraData.substring(0,paraData.length-1);
			}
			var currentValueArray = paraData.split(",");
			var valueCache = {};
			for(var i=0;i<currentValueArray.length;i++){
				valueCache[currentValueArray[i]] = true;
			}
			wrapperElemObj.find(".checkbox-item").each(function(){
				var radioItem = $(this);
				var inputItem = radioItem.find(".checkbox-input");
				if(valueCache[radioItem.find(".text").attr("val")])inputItem.addClass("checked");
				else inputItem.removeClass("checked");
			});
		}
		
}