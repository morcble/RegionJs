//多项输入框
var ItemsinputPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var hide = tmpObj.attr("hide");
			if("true"==hide){
				//tmpObj.attr("type","hidden");
				tmpObj.closest(".data-block-wrapper").addClass("hidden");
			}
			
			tmpObj.addClass("wrapped");
			tmpObj.attr("type","hidden");
			var wrappedHtml = tmpObj.prop("outerHTML");
			
			tmpObj.attr("type","text");
			tmpObj.removeClass("wrapped");
			tmpObj.attr("region-attr",null);
			tmpObj.removeClass("region-editable");
			tmpObj.addClass("inputarea");
			var width = tmpObj.attr("width");
			if(width!=null){
				tmpObj.css("width",width);
			}

			var wrappedLabel = tmpObj.attr("label");//标签自带的label
			
			var replaceHtml = null;
			if(wrappedLabel==null){
				replaceHtml = '<div class="items-input-wrapper region-wrapper">'+wrappedHtml+tmpObj.prop("outerHTML");
			}
			else{
				replaceHtml = '<div class="items-input-wrapper region-wrapper"><span class="wrapped-label">'+wrappedLabel+'</span>'+wrappedHtml+tmpObj.prop("outerHTML");
			}

			replaceHtml += '	<div class="items"></div>';
			replaceHtml+='</div>';
			var replaced = $(replaceHtml);
			
			if(width!=null){
				replaced.css("width",width);
			}
			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			ItemsinputPlugin.renderItems(replaced,paraData);
			
			/*tmpObj = replaced.find(".wrapped");
			tmpObj.val(paraData);*/
			
			replaced.find(".inputarea").keyup(function(event){
				if(event.which==13){
					var newVal = $(this).val().trim();
					if(newVal!=""){
						ItemsinputPlugin.appendItems(replaced,newVal);
					}
				}
			});
			
			replaced.find(".inputarea").blur(function(){
				var newVal = $(this).val().trim();
				if(newVal!=""){
					ItemsinputPlugin.appendItems(replaced,newVal);
				}
			});
			
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			ItemsinputPlugin.renderItems(wrapperElemObj,paraData);
		},
		renderItems:function(wrapperElemObj,itemsStr){//清空并绘制items
			var curRegion = RegionUtil.getRegionByElem(wrapperElemObj[0]);
			
			wrapperElemObj.find(".wrapped").val(itemsStr);
			
			var itemsHtml = "";
			if(itemsStr==null||itemsStr.trim()==""){
				wrapperElemObj.find(".items").html("");
			}
			else{
				var itemValues = itemsStr.split(",");
				for(var i =0;i<itemValues.length;i++){
					if(itemValues[i].trim()=="")continue;
					itemsHtml+='<div class="item">'+itemValues[i]+'<i class="fa-solid fa-remove" onclick="ItemsinputPlugin.removeItem(event)"></i></div>';
				}
				RegionUtil.setInnerHtml(curRegion,wrapperElemObj.find(".items")[0],itemsHtml);
			}
		},
		appendItems:function(wrapperElemObj,itemsStr){//追加节点
			var curRegion = RegionUtil.getRegionByElem(wrapperElemObj[0]);
			
			var curVal = wrapperElemObj.find(".wrapped").val();
			var curValObj = {};
			if(curVal!=null&&curVal.trim()!=""){
				var itemValues = curVal.split(",");//当前值
				for(var i =0;i<itemValues.length;i++){
					if(itemValues[i].trim()=="")continue;
					curValObj[itemValues[i]] = true;
				}
			}
		
			var itemsHtml = "";
			if(itemsStr!=null&&itemsStr.trim()!=""){
				var itemValues = itemsStr.split(",");
				for(var i =0;i<itemValues.length;i++){
					if(itemValues[i].trim()==""||curValObj[itemValues[i]]==true)continue;
					itemsHtml+='<div class="item">'+itemValues[i]+'<i class="fa-solid fa-remove" onclick="ItemsinputPlugin.removeItem(event)"></i></div>';
					curValObj[itemValues[i]] = true;
				}
				RegionUtil.appendHtml(curRegion,wrapperElemObj.find(".items")[0],itemsHtml);
			}
			
			var finalVal = "";
			for(x in curValObj){
				finalVal+=x+",";
			}
			if(finalVal.endWith(","))finalVal = finalVal.substring(0,finalVal.length-1);
			wrapperElemObj.find(".wrapped").val(finalVal);
			wrapperElemObj.find(".inputarea").val("");
		},
		removeItem:function(event){
			var eventTarget = RegionUtil.getEventTarget(event);
			var itemObj = $(eventTarget).parent();
			
			var toDeleteItemValue = itemObj.text();
			var wrapperElemObj = itemObj.closest(".items-input-wrapper");
		
			var curVal = wrapperElemObj.find(".wrapped").val();
			var curValObj = {};
			if(curVal!=null&&curVal.trim()!=""){
				var itemValues = curVal.split(",");//当前值
				for(var i =0;i<itemValues.length;i++){
					if(itemValues[i].trim()==""||toDeleteItemValue==itemValues[i])continue;
					curValObj[itemValues[i]] = true;
				}
			}
			
			var finalVal = "";
			for(x in curValObj){
				finalVal+=x+",";
			}
			if(finalVal.endWith(","))finalVal = finalVal.substring(0,finalVal.length-1);
			wrapperElemObj.find(".wrapped").val(finalVal);
			itemObj.remove();
		}
}