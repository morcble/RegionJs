<style type="text/css">
#REGION{
	position: relative;
	-webkit-touch-callout:none;-webkit-user-select:none;-khtml-user-select:none;-moz-user-select:none;-ms-user-select:none;user-select:none;
}
#REGION .calendar{
/* 	box-shadow: 0 2px 4px 0 rgb(0 0 0 / 10%), 0 2px 5px 0 rgb(0 0 0 / 9%); */
	box-shadow: 0 0 0;
	padding-bottom: 0.5rem;
	position: relative;
}

#REGION .calendar .body{
    overflow: hidden;
}

#REGION .calendar .day.notallowed{
	cursor: not-allowed;
	background: #F9F9F9
}

#REGION .calendar .day.selected{
	background: #dce5f4;
}
</style>
<div id="REGION" class="hidden">
</div>	
<script type="text/javascript">
var REGION = {
		init:function(){
			var curRegion = this;
			var initDate = curRegion.paraMap.get("initDate");//"2008-08-08"
			var dateFormat = curRegion.paraMap.get("dateFormat");
			if(dateFormat==null)dateFormat = "YYYY-MM-DD";
			curRegion.dateFormat = dateFormat;
			
			curRegion.minDate = curRegion.paraMap.get("minDate");//可选最小日期
			curRegion.maxDate = curRegion.paraMap.get("maxDate");//可选最大日期
			var calStyle = curRegion.paraMap.get("calStyle");
			if(calStyle!=null)curRegion.calStyle = calStyle;
			
			var locale = curRegion.paraMap.get("locale");
			if(locale!=null)curRegion.locale = locale;
			
			var exclude = curRegion.paraMap.get("exclude");//不能选择的日期 格式如["2023-04-03","2023-04-06:2023-04-08"]
			if(exclude!=null){
				exclude = JSON.parse(exclude);
				
				curRegion.exclude = {};
				curRegion.excludeRange = [];
				for(var i = 0 ; i<exclude.length ;i++){
					var tmpArray = exclude[i].split(":");
					if(tmpArray.length==0)continue;
					if(tmpArray.length==1)curRegion.exclude[tmpArray] = true;
					else {//start dt to end dt
						curRegion.excludeRange.push({"start":tmpArray[0],"end":tmpArray[1]});
					}
				}
			}
			
			
			//日期主体结构
			var calendarObjHtml = ("<div  class=\"calendar\">" +
					"		<div class=\"cal-header\">" +
					"			<div class=\"date-block\">" +
					"				<i class=\"pre-year fa fa-angle-left\"></i>" +
					"				<div title=\"点击修改\" class=\"year\">	" +
					"				</div>" +
					"				<i class=\"next-year fa fa-angle-right\"></i>" +
					"			</div>" +
					"			" +
					"			<div class=\"date-block\">" +
					"				<i class=\"pre-month fa fa-angle-left\"></i>" +
					"				<div title=\"点击修改\" class=\"month\">" +
					"				</div>" +
					"				<i class=\"next-month fa fa-angle-right\"></i>" +
					"			</div>" +
					"<div class=\"close-block\"><i class=\"fa-solid fa-remove\"></i></div>" +
					"		</div>" +
					"		<div class=\"spliter\"></div>" +
					"		<div class=\"body\"></div>" +
					"	</div>");
			RS.setInnerHtml(curRegion,this.getRegionDivElem(),calendarObjHtml);//绘制date
			this.calendarObj = curRegion.find(".calendar"); 
			
			curRegion.dateValue = moment(initDate,curRegion.dateFormat);
			if(!curRegion.dateValue.isValid()){
				curRegion.dateValue = moment();
			}
			
			curRegion.navStatus = {};//缓存顶部输入的年月
			curRegion.navStatus.year = curRegion.dateValue.year();
			curRegion.navStatus.month = curRegion.dateValue.month();
			//curRegion.navStatus.day = curRegion.dateValue.date();
		},
		renderCalendar:function(){
			var calendarObj = this.calendarObj;
			var curRegion = this;
			var dayHtml="";
			var weekdays = null;
			//日期列名
			if(curRegion.calStyle==1){
				if(curRegion.locale=="en"){
					weekdays = ["SUN","MON","TUE","WED","THU","FRI","SAT"];
				}
				else if(curRegion.locale=="cn"){
					weekdays = ["日","一","二","三","四","五","六"];
				}
				else{
					weekdays = ["SUN","MON","TUE","WED","THU","FRI","SAT"];
				}
			}
			else{
				if(curRegion.locale=="en"){
					weekdays = ["MON","TUE","WED","THU","FRI","SAT","SUN"];
				}
				else if(curRegion.locale=="cn"){
					weekdays = ["一","二","三","四","五","六","日"];
				}
				else{
					weekdays = ["MON","TUE","WED","THU","FRI","SAT","SUN"];
				}
			}

			for(var i = 0;i <7;i++){
				dayHtml+="<div class='weekday'>"+weekdays[i]+"</div>";
			}

			dayHtml+=curRegion.buildDaysTable();
			RS.setInnerHtml(curRegion,calendarObj.find(".body")[0],dayHtml);//绘制date

			//年输入框
			var yearObj = calendarObj.find(".year");
 			var yearLabel = $("<label></label>");
 			yearLabel.text(curRegion.navStatus.year);
 			RS.setInnerHtml(curRegion,yearObj[0],yearLabel.prop("outerHTML"));
 			
			yearObj.click(REGION.editableClickFunc.bind(curRegion,yearObj,"46px",4,
				function(newVal){
				    curRegion.navStatus.year = parseInt(newVal);
					curRegion.renderCalendar();
				},
				function(){//输入过滤非数字
					var value=$(this).val().replace(/[^\d]/g,'')
					$(this).val(value);
				}
			));
			
			
			//月份输入框
			var monthObj = calendarObj.find(".month");
			var monthLabel = $("<label></label>");
 			monthLabel.text(curRegion.navStatus.month+1);
 			RS.setInnerHtml(curRegion,monthObj[0],monthLabel.prop("outerHTML"));

			monthObj.click(REGION.editableClickFunc.bind(curRegion,monthObj,"30px",2,
				function(newVal){
					if(newVal==0){
						curRegion.navStatus.month = 11;//向前翻一年
						curRegion.navStatus.year = curRegion.navStatus.year -1;
						monthLabel.text(curRegion.navStatus.month+1);
						yearLabel.text(curRegion.navStatus.year);
					}
					else if(newVal>11){//向后翻年
						var x = Math.floor(newVal/12);
						var y = newVal%12;
						if(y>0){
							curRegion.navStatus.month = y-1;
							curRegion.navStatus.year = curRegion.navStatus.year +x;
						}
						else{
							curRegion.navStatus.month=11;
							curRegion.navStatus.year = curRegion.navStatus.year+x-1;
						}
						monthLabel.text(curRegion.navStatus.month+1);
						yearLabel.text(curRegion.navStatus.year);
					}
					else{
						curRegion.navStatus.month = newVal-1;
					}
					curRegion.renderCalendar();
				},
				function(){//输入过滤非数字
					var value=$(this).val().replace(/[^\d]/g,'')
					$(this).val(value);
				}
			));

			calendarObj.find(".day").removeClass("chosen-date");
			if(curRegion.dateValue.year()==curRegion.navStatus.year){
				if(curRegion.dateValue.month()==curRegion.navStatus.month){
					calendarObj.find(".curMonth"+curRegion.dateValue.date()).addClass("chosen-date");
				}
				else if(curRegion.dateValue.month()==(curRegion.navStatus.month-1)){
					calendarObj.find(".preMonth"+curRegion.dateValue.date()).addClass("chosen-date");
				}
				else if(curRegion.dateValue.month()==(curRegion.navStatus.month+1)){
					calendarObj.find(".nextMonth"+curRegion.dateValue.date()).addClass("chosen-date");
				}
			}

			//日期点击事件
			calendarObj.find(".day").click(function(){
				var dayCell = $(this);
				if(dayCell.hasClass("notallowed"))return;

				calendarObj.find(".day").removeClass("chosen-date").removeClass("selected");
				dayCell.addClass("chosen-date");
				
				curRegion.dateValue = curRegion.getCellDate(dayCell);
				
				if(typeof curRegion.onDayClick==="function"){
					curRegion.onDayClick(dayCell,curRegion.dateValue.format(curRegion.dateFormat));
				}
			});
			
			calendarObj.find(".day").mouseover(function(){
				var dayCell = $(this);
				if(dayCell.hasClass("notallowed"))return;
				
				var cellDate = curRegion.getCellDate(dayCell);
				
				if(typeof curRegion.onDayHover==="function"){
					curRegion.onDayHover(dayCell,cellDate);
				}
			});

			//添加日期控制事件
			calendarObj.find(".pre-year").unbind("click");
			calendarObj.find(".pre-year").click(function(){
				curRegion.navStatus.year--;
				curRegion.renderCalendar();
			});

			calendarObj.find(".next-year").unbind("click");
			calendarObj.find(".next-year").click(function(){
				curRegion.navStatus.year++;
				curRegion.renderCalendar();
			});

			calendarObj.find(".pre-month").unbind("click");
			calendarObj.find(".pre-month").click(function(){
				curRegion.navStatus.month--;
				if(curRegion.navStatus.month==-1){
					curRegion.navStatus.month=11;
					curRegion.navStatus.year--;
				}
				curRegion.renderCalendar();
			});

			calendarObj.find(".next-month").unbind("click");
			calendarObj.find(".next-month").click(function(){
				curRegion.navStatus.month++;
				if(curRegion.navStatus.month==12){
					curRegion.navStatus.month=0;
					curRegion.navStatus.year++;
				}
				curRegion.renderCalendar();
			});
		},
		editableClickFunc:function(editableObj,width,maxlength,editCallBack,onInputFunc){//年和月的输入函数
			if(maxlength==null)maxlength=-1;
			var editableInput=$("<input type='text' maxlength='"+maxlength+"' class='editableInput'>");
			editableInput.css("width",width);
			var editableLabel = editableObj.find("label");
			editableLabel.replaceWith(editableInput);
			editableInput.val(parseInt(editableLabel.text()));
			editableInput.focus();
			editableInput.select();

			if(typeof onInputFunc ==="function"){//拦截输入事件
				editableInput.bind('input propertychange',onInputFunc);
			}

			var curRegion = this;
			editableObj.unbind("click");
			var completeInputFunc = function(){
				editableInput.replaceWith(editableLabel);
				var newVal = editableInput.val();
				editableLabel.text(newVal);
				editableObj.click(REGION.editableClickFunc.bind(curRegion,editableObj,width,maxlength,editCallBack));
				if(typeof editCallBack ==="function"){
					editCallBack.call(editableObj,newVal);
				}
			}
			editableInput.blur(function(){
				completeInputFunc();
			});
			editableInput.keydown(function(){
				if(event.which==13){
					completeInputFunc();
				}
			});
		},
		buildDaysTable:function(){
			var curRegion = this;
			var resultHtml = "";
			var startDayOfMonth = curRegion.getCalendarTime().startOf('month');//当月开始的日期对象
			var endDayOfMonth = curRegion.getCalendarTime().endOf('month');//当月结束的日期对象
			var start = startDayOfMonth.format('E');//本月第一天的序号
			var end = endDayOfMonth.format('E');

			
			if(curRegion.minDate!=null){//有最小日期
				curRegion.minDateMoment = moment(curRegion.minDate,curRegion.dateFormat);
			}
			if(curRegion.maxDate!=null){//有最大日期
				curRegion.maxDateMoment = moment(curRegion.maxDate,curRegion.dateFormat);
			}
			
			var prependFromLastMonth = 0;
			if(curRegion.calStyle==1){//星期日开头
				if(start!=7){//如果第一天不是星期日 则把上个月的最后几天拿来补足
					prependFromLastMonth = start;//需要从上个月补的天数

				}
			}
			else{//星期一开头
				if(start!=1){//如果第一天不是星期一 则把上个月的最后几天拿来补足
					prependFromLastMonth = start-1;//需要从上个月补的天数
				}
			}
			var endDayOfLastMonth = curRegion.getCalendarTime().month(curRegion.getCalendarTime().month() - 1).endOf('month');
			var numOfEndDayOfLastMonth= endDayOfLastMonth.format('DD');//上个月最后一天的日期
			
			var currentDay = moment();
			currentDay.year(curRegion.navStatus.year);
			currentDay.month(curRegion.navStatus.month-1);
			for(var i = prependFromLastMonth;i >0;i--){
				var dayTxt = numOfEndDayOfLastMonth-i+1;
				currentDay.date(dayTxt);
				var disableDay = false;
				if(curRegion.minDateMoment!=null){
					if(curRegion.minDateMoment.format(curRegion.dateFormat)>currentDay.format(curRegion.dateFormat))disableDay = true;
				}
				if(curRegion.maxDateMoment!=null){
					if(curRegion.maxDateMoment.format(curRegion.dateFormat)<currentDay.format(curRegion.dateFormat))disableDay = true;
				}
				
				if(!disableDay && curRegion.exclude!=null){
					disableDay = curRegion.isExcludeDay(currentDay);
				}
				
				if(disableDay){
					resultHtml+="<div num='"+(numOfEndDayOfLastMonth-i+1)+"' class='day notallowed lastmonth preMonth"+dayTxt+"'>"+(dayTxt)+"</div>";
				}
				else{
					resultHtml+="<div num='"+(numOfEndDayOfLastMonth-i+1)+"' class='day lastmonth preMonth"+dayTxt+"'>"+(dayTxt)+"</div>";
				}
			}


			//绘制当前月的日期单元格
			var daysOfCurrentMonth = endDayOfMonth.format('DD') - startDayOfMonth.format('DD')+1;//当月总天数
			var dayTxt = null;
			//var currentDay = moment();
			currentDay.year(curRegion.navStatus.year);
			currentDay.month(curRegion.navStatus.month);
			
			for(var i = 0;i <daysOfCurrentMonth;i++){
				dayTxt = (i+1);//默认为数字
				currentDay.date(dayTxt);
				
				if(curRegion.specialDays!=null){
					for(x in curRegion.specialDays){
						var tmpSprcialDay = curRegion.specialDays[x];
						var tmpFormat = tmpSprcialDay.format;
						if(tmpFormat==null)tmpFormat=curRegion.dateFormat;

						var tmpDateStr = currentDay.format(tmpFormat);
						if(tmpDateStr==tmpSprcialDay.date){
							dayTxt = tmpSprcialDay.desc;
							break;
						}
					}
				}
				
				var disableDay = false;
				if(curRegion.minDateMoment!=null){
					if(curRegion.minDateMoment.format(curRegion.dateFormat)>currentDay.format(curRegion.dateFormat))disableDay = true;
				}
				if(curRegion.maxDateMoment!=null){
					if(curRegion.maxDateMoment.format(curRegion.dateFormat)<currentDay.format(curRegion.dateFormat))disableDay = true;
				}
				
				if(!disableDay && curRegion.exclude!=null){
					disableDay = curRegion.isExcludeDay(currentDay);
				}
				
				if(disableDay){
					resultHtml+="<div num='"+(i+1)+"' class='day notallowed curMonth"+(i+1)+"'>"+dayTxt+"</div>";
				}
				else{
					resultHtml+="<div num='"+(i+1)+"' class='day curMonth"+(i+1)+"'>"+dayTxt+"</div>";
				}
			}

			var appendFromNextMonth = 0;
			if(curRegion.calStyle==1){
				if(end!=6){//如果当前月最后一天不是星期六 则拿下个月的前几天补齐
					appendFromNextMonth = 7-end-1;
					if(appendFromNextMonth<0)appendFromNextMonth+=7;
				}
			}
			else{
				if(end!=7){//如果当前月最后一天不是星期日 则拿下个月的前几天补齐
					appendFromNextMonth = 7 - end;
				}
			}
			
			currentDay.year(curRegion.navStatus.year);
			currentDay.month(curRegion.navStatus.month+1);
			for(var i = appendFromNextMonth,j=1;i >0;i--,j++){
				var dayTxt = j;
				currentDay.date(dayTxt);
				var disableDay = false;
				if(curRegion.minDateMoment!=null){
					if(curRegion.minDateMoment.format(curRegion.dateFormat)>currentDay.format(curRegion.dateFormat))disableDay = true;
				}
				if(curRegion.maxDateMoment!=null){
					if(curRegion.maxDateMoment.format(curRegion.dateFormat)<currentDay.format(curRegion.dateFormat))disableDay = true;
				}
				
				if(!disableDay && curRegion.exclude!=null){
					disableDay = curRegion.isExcludeDay(currentDay);
				}
				
				if(disableDay){
					resultHtml+="<div  num='"+(j)+"' class='day notallowed nextmonth nextMonth"+dayTxt+"'>"+dayTxt+"</div>";
				}
				else{
					resultHtml+="<div  num='"+(j)+"' class='day nextmonth nextMonth"+dayTxt+"'>"+dayTxt+"</div>";
				}
			}

			return resultHtml;
		},
		getCalendarTime:function(){
			var curRegion = this;
			var result = moment();
			if(curRegion.navStatus.year!=null){
				result.year(curRegion.navStatus.year);
			}
			if(curRegion.navStatus.month!=null){
				result.month(curRegion.navStatus.month);
			}
			return result;
		},
		getDate:function(){//获取选中的日期
			if(this.dateValue==null)return null;
			return this.dateValue.format(this.dateFormat);
		},
		getCellDate:function(dayCell){
			var curRegion = this;
			dayCell.hasClass("lastmonth");
			
			var year = curRegion.navStatus.year;
			var month = null;
			var day = null;
			if(dayCell.hasClass("lastmonth")){
				month = curRegion.navStatus.month-1;
			}
			else if(dayCell.hasClass("nextmonth")){
				month = curRegion.navStatus.month+1;
			}
			else{
				month = curRegion.navStatus.month;
			}
			
			if(curRegion.navStatus.month==-1){
				month = 11;
				year = curRegion.navStatus.year-1;
			}
			else if(curRegion.navStatus.month==12){
				month=0;
				year = curRegion.navStatus.year+1;
			}
			day = parseInt(dayCell.attr("num"));
			
			var cellDay = moment();
			cellDay.year(year);
			cellDay.month(month);
			cellDay.date(day);
			cellDay.hour(0);
			cellDay.minute(0);
			cellDay.second(0);
			return cellDay;
		},
		isExcludeDay:function(dateMoment){
			var curRegion = this;
			var curDarStr = dateMoment.format(curRegion.dateFormat);
			if(curRegion.exclude[curDarStr]==true){
				return true;
			}
			else{
				for(var k = 0 ; k<curRegion.excludeRange.length ;k++){
					var tmpRange = curRegion.excludeRange[k];
					if(tmpRange.start<=curDarStr && curDarStr<=tmpRange.end){
						return true;
					}
				}
				return false;
			}
		},
		afterRenderData:function(){
			var curRegion = this;
			//var paraName = this.paraMap.get("paraName");
			curRegion.init();
			curRegion.renderCalendar();
		}
};


RegionUtil.ready(function(){
	var dateRegion = RegionUtil.newStaticRegion("#REGION",null);
	dateRegion.afterRenderData = REGION.afterRenderData;
	dateRegion.init = REGION.init;
	dateRegion.getCalendarTime = REGION.getCalendarTime;//获取日期表格界面的时间
	dateRegion.renderCalendar = REGION.renderCalendar;//渲染日期表格
	dateRegion.buildDaysTable = REGION.buildDaysTable;//拼接日期表格
	dateRegion.getCellDate = REGION.getCellDate;//获取day cell的日期
	dateRegion.isExcludeDay = REGION.isExcludeDay;//判断日期是否是排除掉的
	
	dateRegion.getDate = REGION.getDate;//获取日期
	dateRegion.dateValue = null;//缓存日历
	
	dateRegion.calStyle= 1;//2  首列是星期日还是星期一
	dateRegion.locale = "cn";   //cn  en
// 	dateRegion.specialDays = [{
// 		"date":"04-13",
// 		"desc":"儿童节",
// 		"format":"MM-DD"
// 	}];//特殊日期标注

// 	dateRegion.onDayClick = function(dateCell,date){//鼠标点击DAY
// 			alert(1)
// };
	
	dateRegion.renderRegion();
})
</script>


							