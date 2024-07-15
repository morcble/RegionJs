<style>
#REGION{
	border: 1px solid #C9C9C9;
	background: white;
}
#REGION .footer {
  text-align: right;
  padding: 0.3125rem 0.625rem 0.625rem 0;
}
#REGION .footer span {
  border: 1px solid #C9C9C9;
  font-size: 0.875rem;
  padding: 0.125rem 0.5rem;
  cursor: pointer;
  border-radius: 0.1875rem;
  display: inline-block;
  line-height: 1.5625rem;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
#REGION .footer span:hover {
  background: #1663f1;
  color: white;
}
</style>
<div id="REGION" class="hidden">
	<div class="dateHolder"></div>
	<div class="footer">
		<span class="clear-btn"></span> 
		<span class="today-btn hidden"></span>
<!-- 		<span class="ok-btn"></span> -->
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		init:function(){
			var curRegion = this;
			
			var noTodayBtn = curRegion.paraMap.get("noTodayBtn");
			if(noTodayBtn!="true"){
				curRegion.find(".today-btn").removeClass("hidden");
			}
			
			var loadDatePromise = RS.loadRegion(curRegion.find(".dateHolder"),RS.appendParaMapToUrl("common/shared/date/core/date.rs",curRegion.paraMap));
			loadDatePromise.done(function(dateRegion){
// 				curRegion.find(".ok-btn").click(function(){
// 					if(typeof curRegion.onOkClick==="function"){
// 						curRegion.onOkClick(dateRegion.getDate());
// 					}
// 				});
				
				curRegion.find(".today-btn").click(function(){
					var nowTimeStr = moment().format(dateRegion.dateFormat);
					
					if(typeof curRegion.onNowClick==="function"){
						curRegion.onNowClick(nowTimeStr);
					}
				});
				
				curRegion.find(".clear-btn").click(function(){
					if(typeof curRegion.onClearClick==="function"){
						curRegion.onClearClick();
					}
				});
				
				dateRegion.onDayClick = function(dateCell,date){
					if(typeof curRegion.onDayClick==="function"){
						curRegion.onDayClick(date);
					}
				}
			});

			
			curRegion.find(".today-btn").text(global_msg.today);
			curRegion.find(".ok-btn").text(global_msg.ok);
			curRegion.find(".clear-btn").text(global_msg.clear);
			
			return loadDatePromise;
		},
		afterRenderData:function(){
			var curRegion = this;
			return curRegion.init();
		}
};
RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.init = REGION.init;
	staticRegion.onOkClick = REGION.onOkClick;
	staticRegion.onNowClick = REGION.onNowClick;
	staticRegion.renderRegion();
})
</script>