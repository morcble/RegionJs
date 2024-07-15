var MultiSelectPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var isDisabled = tmpObj.hasClass("disabled");
			
			var regionDsName = tmpObj.attr("region-ds");
			var regionAttr = tmpObj.attr("region-attr");
			var className = tmpObj[0].className+" wrapped";
			var options = RegionUtil.getRegionDsList(region,regionDsName);
			
			if(paraData==null)paraData="";
			var optionsHtml = "";
			
			paraData+="";
			
			if(paraData.endWith(",")){
				paraData=paraData.substring(0,paraData.length-1);
			}
			var currentValueArray = paraData.split(",");
			var valueCache = {};
			for(var i=0;i<currentValueArray.length;i++){
				valueCache[currentValueArray[i]] = true;
			}
			
			tmpObj.find("option").each(function () {
	            optionsHtml+="<li val='"+$(this).val()+"'><input class='checkitem' type=\"checkbox\"><font class='label-txt'>"+$(this).text()+"</font></li>"; 
	        })
			
			//inner html begin			
			var width = tmpObj.attr("width");
			var height = tmpObj.attr("height");
			if(width==null||width=="")width="500px";//默认宽度
			if(height==null||height=="")height="200px";//默认高度

			var replaceHtml = '<div class="multiselect-wrapper region-wrapper">';
			replaceHtml+='<input type="hidden" class="'+className+'" region-ds="'+regionDsName+'" region-attr="'+regionAttr+'" onchange="'+onchange+'">';
			replaceHtml+='	<div class="options-panel">';
			replaceHtml+='		<div class="options scroll-bar1">';
			
			tmpObj.find("option").each(function () {
				replaceHtml+='<li><input class="checkitem" type="checkbox" value="'+$(this).val()+'"><span>'+$(this).text()+'</span></li>';
	        })
			if(options!=null){
				for(var i = 0 ; i <options.length;i++){
					replaceHtml+='<li><input class="checkitem" type="checkbox" value="'+options[i].value+'"><span>'+options[i].text+'</span></li>';
				}
			}
			replaceHtml+='		</div>';
			replaceHtml+='	</div>';
			replaceHtml+='</div>';		
			
			var selectWrapper = $(replaceHtml);
			RegionUtil.addRegionUniqueId(selectWrapper[0],region.regionId);
			selectWrapper.css("width",width);
			selectWrapper.css("height",height);

			tmpObj.replaceWith(selectWrapper);
			if(isDisabled){
				selectWrapper.addClass("disabled");
			}
			
			var initCheckItems = selectWrapper.find(".checkitem");
			initCheckItems.each(function(){//代码类似1
				var value = $(this).val();
				if(valueCache[value])
					$(this).prop("checked",true);
			});
			
			selectWrapper.find(".checkitem").click(function(){
				var checkItems = selectWrapper.find(".checkitem");
				var newValue = "";
				checkItems.each(function(){
				    if($(this).prop('checked')){
				    	if(newValue=="")newValue+=$(this).val();
				    	else newValue+=","+$(this).val();
				    }
				});
				selectWrapper.find(".wrapped").val(newValue);
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(this,newValue)');
					//eval(onchange+"('"+newValue+"')");
				}
			});
			
			tmpObj = selectWrapper.find(".wrapped");
			tmpObj.val(paraData);
			return tmpObj;
		},
		refreshDataSource:function(region,wrapperElemObj,regionElemObj,options){//动态更新options
			var paraData = regionElemObj.val();
			var onchange = regionElemObj.attr("onchange");
			var selectWrapper = wrapperElemObj;
			var optionsHtml = "";
			if(options!=null){
				for(var i = 0 ; i <options.length;i++){
					optionsHtml+='<li><input class="checkitem" type="checkbox" value="'+options[i].value+'"><span>'+options[i].text+'</span></li>';
				}
			}
			selectWrapper.find(".options").html(optionsHtml);
			RegionUtil.addRegionUniqueId(selectWrapper[0],region.regionId);
			selectWrapper.find(".checkitem").click(function(){
				var checkItems = selectWrapper.find(".checkitem");
				var newValue = "";
				checkItems.each(function(){
				    if($(this).prop('checked')){
				    	if(newValue=="")newValue+=$(this).val();
				    	else newValue+=","+$(this).val();
				    }
				});
				selectWrapper.find(".wrapped").val(newValue);
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(this,newValue)');
					//eval(onchange+"('"+newValue+"')");
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
			
			var newValue = "";
			var initCheckItems = selectWrapper.find(".checkitem");
			initCheckItems.each(function(){//代码类似1
				var value = $(this).val();
				if(valueCache[value]){
					$(this).prop("checked",true);
					
					if(newValue=="")newValue+=$(this).val();
			    	else newValue+=","+$(this).val();
				}	
			});
			regionElemObj.val(newValue);
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);

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
			wrapperElemObj.find(".checkitem").each(function(){
				var checkitem = $(this);
				if(valueCache[checkitem.val()])checkitem.prop("checked",true);
				else checkitem.prop("checked",false);
			});
		}
}