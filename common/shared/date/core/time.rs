<style type="text/css">
#REGION{
	position: relative;
	-webkit-touch-callout:none;-webkit-user-select:none;-khtml-user-select:none;-moz-user-select:none;-ms-user-select:none;user-select:none;
}
#REGION .time-chooser{
/* 	box-shadow: 0 2px 4px 0 rgb(0 0 0 / 10%), 0 2px 5px 0 rgb(0 0 0 / 9%); */
	box-shadow: 0 0 0;
	padding-bottom: 0.5rem;
	position: relative;
	height: 19.2rem;
	padding:0rem 0rem;
}

#REGION  .time-chooser .column {
    height: 15.5rem;
}

#REGION  .time-chooser > .time-header {
    height: 2.5rem;
}

</style>

<div id="REGION" class="hidden">

</div>	


<script type="text/javascript">
var REGION = {
		init:function(){
			var curRegion = this;
			var initTimeStr = curRegion.paraMap.get("initTime");
			var timeFormat = curRegion.paraMap.get("timeFormat");
			if(timeFormat==null) timeFormat = "HH:mm:ss";
			curRegion.timeFormat = timeFormat;
			
			var hideSeconds = false;//隐藏秒
			if(curRegion.timeFormat.indexOf("ss")==-1){
				hideSeconds = true;
			}
			curRegion.hideSeconds = hideSeconds;
			
			var hoursHtml = "";
			for(var i = 0 ; i <24;i++){
				var txt = i+"";
				if(txt.length==1)txt="0"+txt;
				hoursHtml+="<div num='"+i+"' class='item item"+i+"'>"+txt+"</div>";
			}
			var minutesHtml = "";
			for(var i = 0 ; i <60;i++){
				var txt = i+"";
				if(txt.length==1)txt="0"+txt;
				minutesHtml+="<div num='"+i+"' class='item item"+i+"'>"+txt+"</div>";
			}
			var secondsHtml = minutesHtml;

			var timeHtml = "<div  class=\"time-chooser\">" +
			"								<div class=\"time-header\">" +
			"									选择时间" +
			"								</div>" +
			"								<div class=\"spliter\"></div>" +
			"								<div class=\"body\">" +
			"<div class='column'><div class='item-name'>时</div><div class='hour item-holder no-scrollbar'>"+hoursHtml+"</div></div>"+
			"<div class='column'><div class='item-name'>分</div><div class='minute item-holder no-scrollbar'>"+minutesHtml+"</div></div>";
			
			if(!hideSeconds)
				timeHtml +="<div class='column'><div class='item-name'>秒</div><div class='second item-holder no-scrollbar'>"+secondsHtml+"</div></div>";
			
			timeHtml +="</div>" +" </div>";

			RS.setInnerHtml(curRegion,this.getRegionDivElem(),timeHtml);//绘制date
			
			var timeHolderObj = curRegion.find(".time-chooser");
			timeHolderObj.find(".item").click(function(){
				if($(this).hasClass("selected"))return;

				$(this).parent().find(".item").removeClass("selected");
				$(this).addClass("selected");
			});
			
			timeHolderObj.find(".item-holder").mouseover(function(){
				$(this).removeClass("no-scrollbar");
				//$(this).find(".item").css("padding-left","15px");
			});
			timeHolderObj.find(".item-holder").mouseout(function(){
				$(this).addClass("no-scrollbar");
				//$(this).find(".item").css("padding-left","0px");
			});
			
			curRegion.selectItemsByTime(initTimeStr);
		},
		selectItemsByTime:function(timeStr){
			var curRegion = this;
			var timeHolderObj = curRegion.find(".time-chooser");
			var hour = 0;
			var minute = 0;
			var second = 0;
			
			var targetDate = null;
			if(timeStr!=null&&timeStr!=""){
				targetDate = moment(timeStr,this.timeFormat);
				if(!targetDate.isValid())targetDate = moment();
			}
			else{
				targetDate = moment();
			}
			
			hour = targetDate.hour();
			minute = targetDate.minute();
			if(!curRegion.hideSeconds)second = targetDate.second();

			var hourPart = timeHolderObj.find(".hour");
			var minutePart = timeHolderObj.find(".minute");
			var secondPart = timeHolderObj.find(".second");

			timeHolderObj.find(".item").removeClass("selected");

			hourPart.find(".item"+hour).addClass("selected");
			minutePart.find(".item"+minute).addClass("selected");
			if(!curRegion.hideSeconds)secondPart.find(".item"+second).addClass("selected");

			var itemHeight = (timeHolderObj.find(".item").height());
			
			hourPart.animate({
			    scrollTop: (hour-2)*itemHeight
			},100);
			minutePart.animate({
			    scrollTop: (minute-2)*itemHeight
			},100);
			if(!curRegion.hideSeconds){
				secondPart.animate({
				    scrollTop: (second-2)*itemHeight
				},100);
			}
		},
		getTime:function(){
			var curRegion = this;
			var timeHolderObj = curRegion.find(".time-chooser");
			var hourPart = timeHolderObj.find(".hour");
			var minutePart = timeHolderObj.find(".minute");
			var secondPart = timeHolderObj.find(".second");

			var hour = hourPart.find(".selected").attr("num");
			var minute = minutePart.find(".selected").attr("num");
			var second = secondPart.find(".selected").attr("num");

			var chosenTimeStr = null;
			if(curRegion.hideSeconds){
				chosenTimeStr = moment().hour(hour).minute(minute).format(curRegion.timeFormat);
			}
			else{
				chosenTimeStr = moment().hour(hour).minute(minute).second(second).format(curRegion.timeFormat);
			}
			return chosenTimeStr;
		},
		afterRenderData:function(){
			var curRegion = this;
			curRegion.init();
		}
};
RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.init = REGION.init;
	staticRegion.selectItemsByTime = REGION.selectItemsByTime;
	staticRegion.getTime = REGION.getTime;//获取时间
	staticRegion.renderRegion();
})
</script>