var TimePickerPlugin = {
		render:function(region,tmpObj,paraData){
			var width = tmpObj.attr("width");
			tmpObj.addClass("wrapped");

			var replaceHtml = '<div class="timepicker-wrapper region-wrapper">'+tmpObj.prop("outerHTML");
			replaceHtml+='<div></div>'
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			if(width!=null){
				replaced.css("width",width);
			}
			
			tmpObj = replaced.find(".wrapped");
			

			var formatStr = tmpObj.attr("format");
			if(formatStr==null||formatStr==""){
				formatStr = "HH:mm:ss";
			}
			tmpObj.attr("placeholder",formatStr);
			
			var paraMap = new HashMap();
			RS.addDialog(tmpObj,"common/shared/date/core/time-picker.rs",paraMap,
				function(targetInput,paraMap){//可动态调节组件的参数
					paraMap.put("timeFormat",formatStr);//例如修改样式
					paraMap.put("initTime",targetInput.val());//例如修改样式
				},
				function(dialogRegion,targetInput){//消息框显示的时候
					dialogRegion.onOkClick = function(time){
						targetInput.val(time);
						RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
					}
					dialogRegion.onNowClick = function(time){
						targetInput.val(time);
						RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
					}
				});//添加输入悬浮框
			
			
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
			
		}
}