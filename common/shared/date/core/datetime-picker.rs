<style>
#REGION{
	border: 1px solid #C9C9C9;
}
#REGION > .footer {
  text-align: center;
  padding: 0.8rem 0.625rem 1rem 0;
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

#REGION>.dateHolder{
	display: inline-block;
	vertical-align: top;
}

#REGION>.timeHolder{
	display: inline-block;
	vertical-align: top;
}

#REGION .calendar .body{
    height: 16rem;
    overflow: hidden;
}
</style>
<div id="REGION" class="hidden">
	<div class="dateHolder"></div>
	<div class="timeHolder"></div>
	<div class="footer">
		<span class="now-btn"></span> <span class="ok-btn"></span>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		init:function(){
			var curRegion = this;
			
			var jqueryDef = new jQuery.Deferred();
			
			var loadDatePromise = RS.loadRegion(curRegion.find(".dateHolder"),RS.appendParaMapToUrl("common/shared/date/core/date.rs",curRegion.paraMap));
			
			var loadTimePromise = RS.loadRegion(curRegion.find(".timeHolder"),RS.appendParaMapToUrl("common/shared/date/core/time.rs",curRegion.paraMap));

			curRegion.find(".now-btn").text(global_msg.now);
			curRegion.find(".ok-btn").text(global_msg.ok);
			
			$.when(loadDatePromise,loadTimePromise).done(function(dateRegion,timeRegion){
				curRegion.find(".ok-btn").click(function(){
					if(typeof curRegion.onOkClick==="function"){
						curRegion.onOkClick(dateRegion.getDate()+" "+timeRegion.getTime());
					}
				});
			
				curRegion.find(".now-btn").click(function(){
	 				var nowTimeStr = moment().format(dateRegion.dateFormat+" "+timeRegion.timeFormat);
				
					if(typeof curRegion.onNowClick==="function"){
						curRegion.onNowClick(nowTimeStr);
					}
				});
				
				
				
				jqueryDef.resolve();
			});
			
			return jqueryDef.promise();
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