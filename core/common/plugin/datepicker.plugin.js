var DatepickerPlugin = {
		specialDays:[],//全局配置special day
		render:function(region,tmpObj,paraData){
			var width = tmpObj.attr("width");
			
			var noTodayBtn = tmpObj.attr("noTodayBtn");
			
			var onchange = tmpObj.attr("onchange");
			
			tmpObj.addClass("wrapped")
			var replaceHtml = "<div class='calendar-wrapper region-wrapper'>"+tmpObj.prop("outerHTML");
			replaceHtml+='<div class="calendar-icon"><i class="fa-light fa-calendar-alt"></i></div>';
			replaceHtml+='</div>';
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			replaced.find(".calendar-icon").click(function(event){
				var eventDom = RS.getEventTarget(event);
				$(eventDom).closest(".region-wrapper").find(".wrapped").focus();
			})
			
			if(width!=null){
				replaced.css("width",width);
			}
			tmpObj = replaced.find(".wrapped");
			tmpObj.attr("onchange",onchange);

			
			var paraMap = new HashMap();
			if(tmpObj.hasClass("daterange")){
				RS.addDialog(tmpObj,"common/shared/date/core/rangedate-picker.rs",paraMap,
						function(targetInput,paraMap){//可动态调节组件的参数
							paraMap.put("minDate",targetInput.attr("min-date"));
							paraMap.put("maxDate",targetInput.attr("max-date"));
							paraMap.put("calStyle",targetInput.attr("calStyle"));
							paraMap.put("dateFormat",targetInput.attr("format"));
							paraMap.put("locale",targetInput.attr("locale"));
					
							var timeRangeVal = targetInput.val();
							if(timeRangeVal!=null){
								if(timeRangeVal.indexOf("~")!=-1){
									timeRangeVal = timeRangeVal.replaceAll(" ","");
									var timeRangeArray = timeRangeVal.split("~");
									if(timeRangeArray.length==2){
										paraMap.put("startDate",timeRangeArray[0]);
										paraMap.put("endDate",timeRangeArray[1]);
									}
								}
							}
						},
						function(dialogRegion,targetInput){//消息框显示的时候
							var onchange = targetInput.attr("onchange");
							
							dialogRegion.onRangeSelectEnd = function(date){
								targetInput.val(date.startDate+" ~ "+date.endDate);
								
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
						});//添加输入悬浮框
			}
			else if(tmpObj.hasClass("datetime")){//date and time
				RS.addDialog(tmpObj,"common/shared/date/core/datetime-picker.rs",paraMap,
						function(targetInput,paraMap){//可动态调节组件的参数
							var dateFormat = targetInput.attr("dateFormat");
							if(dateFormat==null) dateFormat = "YYYY-MM-DD";
							
							var timeFormat = targetInput.attr("timeFormat");
							if(timeFormat==null) timeFormat = "HH:mm";
					
							paraMap.put("minDate",targetInput.attr("min-date"));
							paraMap.put("maxDate",targetInput.attr("max-date"));
							paraMap.put("calStyle",targetInput.attr("calStyle"));
							paraMap.put("dateFormat",dateFormat);
							paraMap.put("timeFormat",timeFormat);
							paraMap.put("locale",targetInput.attr("locale"));
							
							var curDateTime = targetInput.val();
							var curDateTimeMoment = moment(curDateTime,dateFormat+" "+timeFormat);
							if(curDateTimeMoment.isValid()){
								paraMap.put("initDate",curDateTimeMoment.format(dateFormat));
								paraMap.put("initTime",curDateTimeMoment.format(timeFormat));
							}
						},
						function(dialogRegion,targetInput){//消息框显示的时候
							var onchange = targetInput.attr("onchange");
							dialogRegion.onOkClick = function(time){
								targetInput.val(time);
								
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
							dialogRegion.onNowClick = function(time){
								targetInput.val(time);
								
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
						});//添加输入悬浮框
			}
			else{//only date
				paraMap.put("noTodayBtn",noTodayBtn);
				RS.addDialog(tmpObj,"common/shared/date/core/date-picker.rs",paraMap,
						function(targetInput,paraMap){//可动态调节组件的参数
							paraMap.put("minDate",targetInput.attr("min-date"));
							paraMap.put("maxDate",targetInput.attr("max-date"));
							paraMap.put("calStyle",targetInput.attr("calStyle"));
							paraMap.put("dateFormat",targetInput.attr("format"));

							paraMap.put("initDate",targetInput.val());//例如修改样式
							paraMap.put("locale",targetInput.attr("locale"));
						},
						function(dialogRegion,targetInput){//消息框显示的时候
							var onchange = targetInput.attr("onchange");

							dialogRegion.onOkClick = function(date){
								targetInput.val(date);
								
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
							dialogRegion.onNowClick = function(date){
								targetInput.val(date);
								
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
							dialogRegion.onClearClick = function(date){
								targetInput.val("");
								
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
							dialogRegion.onDayClick = function(date){
								targetInput.val(date);
	
								if(onchange!=null&&onchange!=""){
									eval(onchange+'.call(targetInput[0],targetInput.val())');
								}
								RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
							}
						});//添加输入悬浮框
			}
			
			
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
		}	
}
