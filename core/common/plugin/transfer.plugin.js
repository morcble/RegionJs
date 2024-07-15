var TransferPlugin = {
		render:function(region,tmpObj,paraData){
			//inner html begin
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			var width = tmpObj.attr("width");
			var height = tmpObj.attr("height");
			if(width==null||width=="")width="500px";//默认宽度
			if(height==null||height=="")height="200px";//默认高度
			
			var hiddenInputHtml = tmpObj.prop("outerHTML");
			
			var sourceArray = [];
			var resultArray = [];
			var fieldConfig = region.getAttrConfig([tmpObj.attr("region-attr")]);
			if(fieldConfig==null)
				fieldConfig = region.regionData[tmpObj.attr("region-attr")+"List"];
			if(fieldConfig!=null){
				var paraArray = null;
				if(paraData!=null)paraArray = paraData.split(",");
				
				var paraObj ={};
				if(paraArray!=null){
					for(var i = 0 ;i<paraArray.length;i++){
						paraObj[paraArray[i]] = true;
					}
				}
				
				
				for(var i = 0 ;i<fieldConfig.length;i++){
					var tmpField = fieldConfig[i];
					
					if(paraObj[tmpField["value"]]){
						resultArray.push(tmpField);
					}
					else{
						sourceArray.push(tmpField);
					}
				}
			}
			
			
			var sourceCount = sourceArray.length;
			var resultCount = resultArray.length;
			
			var replaceHtml = '<div class="transfer-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml+='	<div class="source transfer-panel">';
			replaceHtml+='		<div class="header">';
			replaceHtml+='			<li><input class="checkAll" type="checkbox""><span>'+tmpObj.attr("source-title")+'</span><span class="summary">0/'+sourceCount+'</span></li>';
			replaceHtml+='		</div>';
			replaceHtml+='		<div class="body scroll-bar1">';
			if(sourceCount>0){
				var tmpArray = sourceArray;
				for(var i=0;i<tmpArray.length;i++){
					replaceHtml+='<li><input class="checkitem" type="checkbox" value="'+tmpArray[i].value+'"><span>'+tmpArray[i].text+'</span></li>';
				}
			}
			replaceHtml+='		</div>';
			replaceHtml+='	</div>';
			
			replaceHtml+='	<div class="control">';
			replaceHtml+='		<div class="trans-btns">';
			replaceHtml+='			<div><i class="trans-btn trans-right fa-solid fa-arrow-right"></i></div>';
			replaceHtml+='			<div><i class="trans-btn trans-left fa-solid fa-arrow-left"></i></div>';
			replaceHtml+='		</div>';
			replaceHtml+='	</div>';

			replaceHtml+='	<div class="result transfer-panel">';
			replaceHtml+='		<div class="header">';
			replaceHtml+='			<li><input class="checkAll" type="checkbox"><span>'+tmpObj.attr("result-title")+'</span><span class="summary">0/'+resultCount+'</span></li>';
			replaceHtml+='		</div>';
			replaceHtml+='		<div class="body scroll-bar1">';
			if(resultCount>0){
				var tmpArray = resultArray;
				for(var i=0;i<tmpArray.length;i++){
					replaceHtml+='<li><input class="checkitem" type="checkbox" value="'+tmpArray[i].value+'"><span>'+tmpArray[i].text+'</span></li>';
				}
			}
			replaceHtml+='		</div>';
			replaceHtml+='	</div>';
			replaceHtml+='</div>';		
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			replaced.css("width",width);
			replaced.css("height",height);
			replaced.find(".control").css("line-height",height);
			
			tmpObj.replaceWith(replaced);
			//inner html end
			var sourcePart = replaced.find(".source");
			sourcePart.find(".checkAll").click(function(){
				var isChecked = $(this).prop('checked');
				var checkItems = sourcePart.find(".checkitem");
				checkItems.prop('checked',isChecked);
				var amount = checkItems.length;
				if(isChecked)sourcePart.find(".summary").text(amount+'/'+amount);
				else sourcePart.find(".summary").text('0/'+amount);
			});
			
			sourcePart.find(".checkitem").click(function(){
				$(this).prop("checked",!$(this).prop("checked"));
			});
			
			sourcePart.find("li").click(function(){
				var tmpCheckItem = $(this).find(".checkitem");
				if(tmpCheckItem.prop("checked"))tmpCheckItem.prop("checked",false);
				else tmpCheckItem.prop("checked",true);
				
				TransferPlugin.recalculateSourcePart(sourcePart);
			});
			
			var resultPart = replaced.find(".result");
			resultPart.find(".checkAll").click(function(){
				var isChecked = $(this).prop('checked');
				var checkItems = resultPart.find(".checkitem");
				checkItems.prop('checked',isChecked);
				var amount = checkItems.length;
				if(isChecked)resultPart.find(".summary").text(amount+'/'+amount);
				else resultPart.find(".summary").text('0/'+amount);
			});
			
			resultPart.find(".checkitem").click(function(){
				$(this).prop("checked",!$(this).prop("checked"));
			});
			resultPart.find("li").click(function(){
				var tmpCheckItem = $(this).find(".checkitem");
				if(tmpCheckItem.prop("checked"))tmpCheckItem.prop("checked",false);
				else tmpCheckItem.prop("checked",true);
				
				TransferPlugin.recalculateResultPart(resultPart);
			});
			
			replaced.find(".trans-right").click(function(){
				var checkItems = sourcePart.find(".checkitem");
				var changedHtml = "";
				checkItems.each(function(){
				    if($(this).prop('checked')){
				    	var itemObj = $(this).parent();
				    	itemObj.remove();
				    	itemObj.find(".checkitem").prop('checked',false);
				    	changedHtml+=itemObj.prop("outerHTML");
				    }
				});
				
				resultPart.find(".checkAll").prop("checked",false);
				sourcePart.find(".checkAll").prop("checked",false);
				
				if(changedHtml!=""){
					resultPart.find(".body").prepend(changedHtml);
					TransferPlugin.recalculateSourcePart(sourcePart);
					TransferPlugin.recalculateResultPart(resultPart);
					TransferPlugin.calculateResultData(resultPart,replaced);
					
					resultPart.find(".body .checkitem").unbind("click");
					resultPart.find(".body .checkitem").click(function(){
						$(this).prop("checked",!$(this).prop("checked"));
					});
					resultPart.find(".body li").unbind("click");
					resultPart.find(".body li").click(function(){
						var tmpCheckItem = $(this).find(".checkitem");
						if(tmpCheckItem.prop("checked"))tmpCheckItem.prop("checked",false);
						else tmpCheckItem.prop("checked",true);
						
						TransferPlugin.recalculateResultPart(resultPart);
					});
				}
			});
			replaced.find(".trans-left").click(function(){
				var checkItems = resultPart.find(".checkitem");
				var changedHtml = "";
				checkItems.each(function(){
				    if($(this).prop('checked')){
				    	var itemObj = $(this).parent();
				    	itemObj.remove();
				    	itemObj.find(".checkitem").prop('checked',false);
				    	changedHtml+=itemObj.prop("outerHTML");
				    }
				});
				
				resultPart.find(".checkAll").prop("checked",false);
				sourcePart.find(".checkAll").prop("checked",false);
				
				if(changedHtml!=""){
					sourcePart.find(".body").prepend(changedHtml);
					TransferPlugin.recalculateSourcePart(sourcePart);
					TransferPlugin.recalculateResultPart(resultPart);
					TransferPlugin.calculateResultData(resultPart,replaced);
					
					sourcePart.find(".body .checkitem").unbind("click");
					sourcePart.find(".body .checkitem").click(function(){
						$(this).prop("checked",!$(this).prop("checked"));
					});
					sourcePart.find(".body li").unbind("click");
					sourcePart.find(".body li").click(function(){
						var tmpCheckItem = $(this).find(".checkitem");
						if(tmpCheckItem.prop("checked"))tmpCheckItem.prop("checked",false);
						else tmpCheckItem.prop("checked",true);
						
						TransferPlugin.recalculateSourcePart(resultPart);
					});
				}
			});
			
			TransferPlugin.calculateResultData(resultPart,replaced);//计算初始化值
			tmpObj = replaced.find(".transfer");
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			var sourceArray = [];
			var resultArray = [];
			var fieldConfig = region.getAttrConfig([regionElemObj.attr("region-attr")]);
			if(fieldConfig==null)
				fieldConfig = region.regionData[tmpObj.attr("region-attr")+"List"];
			if(fieldConfig!=null){
				var paraArray = null;
				if(paraData!=null)paraArray = paraData.split(",");
				
				var paraObj ={};
				if(paraArray!=null){
					for(var i = 0 ;i<paraArray.length;i++){
						paraObj[paraArray[i]] = true;
					}
				}
				
				
				for(var i = 0 ;i<fieldConfig.length;i++){
					var tmpField = fieldConfig[i];
					
					if(paraObj[tmpField["value"]]){
						resultArray.push(tmpField);
					}
					else{
						sourceArray.push(tmpField);
					}
				}
			}
			
			var sourcePart = wrapperElemObj.find(".source");
			var resultPart = wrapperElemObj.find(".result");
			
			var sourceCount = sourceArray.length;
			var resultCount = resultArray.length;
			
			var replaceHtml = "";
			if(sourceCount>0){
				var tmpArray = sourceArray;
				for(var i=0;i<tmpArray.length;i++){
					replaceHtml+='<li><input class="checkitem" type="checkbox" value="'+tmpArray[i].value+'"><span>'+tmpArray[i].text+'</span></li>';
				}
			}
			
			sourcePart.find(".body").html(replaceHtml);
			replaceHtml = "";
			
			sourcePart.find(".body .checkitem").unbind("click");
			sourcePart.find(".body .checkitem").click(function(){
				$(this).prop("checked",!$(this).prop("checked"));
			});
			
			sourcePart.find(".body li").unbind("click");
			sourcePart.find(".body li").click(function(){
				var tmpCheckItem = $(this).find(".checkitem");
				if(tmpCheckItem.prop("checked"))tmpCheckItem.prop("checked",false);
				else tmpCheckItem.prop("checked",true);
				
				TransferPlugin.recalculateSourcePart(sourcePart);
			});
			
			if(resultCount>0){
				var tmpArray = resultArray;
				for(var i=0;i<tmpArray.length;i++){
					replaceHtml+='<li><input class="checkitem" type="checkbox" value="'+tmpArray[i].value+'"><span>'+tmpArray[i].text+'</span></li>';
				}
			}
			
			resultPart.find(".body").html(replaceHtml);
			
			resultPart.find(".body .checkitem").click(function(){
				$(this).prop("checked",!$(this).prop("checked"));
			});
			resultPart.find(".body li").click(function(){
				var tmpCheckItem = $(this).find(".checkitem");
				if(tmpCheckItem.prop("checked"))tmpCheckItem.prop("checked",false);
				else tmpCheckItem.prop("checked",true);
				
				TransferPlugin.recalculateResultPart(resultPart);
			});
			
			TransferPlugin.recalculateSourcePart(sourcePart);
			TransferPlugin.recalculateResultPart(resultPart);
			TransferPlugin.calculateResultData(resultPart,wrapperElemObj);
		},
		recalculateResultPart:function(resultPart){
			var checkItems = resultPart.find(".checkitem");
			var checkedAmount = 0;
			checkItems.each(function(){
			    if($(this).prop('checked'))checkedAmount++;
			});
			var amount = checkItems.length;
			resultPart.find(".summary").text(checkedAmount+'/'+amount);
		},
		calculateResultData:function(resultPart,wrapperElemObj){
			var checkItems = resultPart.find(".checkitem");
			var resultData = "";
			checkItems.each(function(){
				resultData+=$(this).attr("value")+",";
			});
			if(resultData.length>1)resultData=resultData.substring(0,resultData.length-1);
			wrapperElemObj.find(".transfer").val(resultData);
			//console.log("resultData="+resultData);
		},
		recalculateSourcePart:function(sourcePart){
			var checkItems = sourcePart.find(".checkitem");
			var checkedAmount = 0;
			checkItems.each(function(){
			    if($(this).prop('checked'))checkedAmount++;
			});
			var amount = checkItems.length;
			sourcePart.find(".summary").text(checkedAmount+'/'+amount);
		}
}