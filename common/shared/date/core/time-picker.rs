<style>
#REGION{
	border: 1px solid #C9C9C9;
}
#REGION > .footer {
  text-align: right;
  padding: 0.8rem 0.625rem 1rem 0;
  width: 17.125rem;
}
#REGION > .footer span {
  border: 1px solid #C9C9C9;
  font-size: 0.875rem;
  padding: 0.3rem 0.5rem;
  cursor: pointer;
  border-radius: 0.3rem;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
#REGION > .footer span:hover {
  background: #1663f1;
  color: white;
}
</style>
<div id="REGION" class="hidden">
	<div class="timeHolder"></div>
	<div class="footer">
		<span class="now-btn"></span> <span class="ok-btn"></span>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		init:function(){
			var curRegion = this;
			var loadTimePromise = RS.loadRegion(curRegion.find(".timeHolder"),RS.appendParaMapToUrl("common/shared/date/core/time.rs",curRegion.paraMap));
			loadTimePromise.done(function(timeRegion){
				curRegion.find(".ok-btn").click(function(){
					if(typeof curRegion.onOkClick==="function"){
						curRegion.onOkClick(timeRegion.getTime());
					}
				});
				
				curRegion.find(".now-btn").click(function(){
	 				var hour = moment().hour();
	 				var minute = moment().minute();
	 				var second = moment().second();

	 				if(hour.length==1)hour="0"+hour;
	 				if(minute.length==1)minute="0"+minute;
	 				if(second.length==1)second="0"+second;

	 				var nowTimeStr = moment().format(timeRegion.timeFormat);
					
					if(typeof curRegion.onNowClick==="function"){
						curRegion.onNowClick(nowTimeStr);
					}
				});
			});

			curRegion.find(".now-btn").text(global_msg.now);
			curRegion.find(".ok-btn").text(global_msg.ok);
			
			return loadTimePromise;
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