<div id="REGION" class="hidden" style="text-align: center;">
	<div class="alert-content"><span style="position: relative;top: 20%" region-attr="msg"></span></div>
	<button class="btn " data-type="primary" normal style="margin-right: 30px;" onclick='AlertPlugin.yes(event)'><message key="global_msg.yes"></message></button>
	<button class="btn " data-type="default" normal onclick='AlertPlugin.no(event)'><message key="global_msg.no"></message></button>
</div>	
<script type="text/javascript">
RegionUtil.ready(function(){
	var region = RegionUtil.newStaticRegion("#REGION");
	region.renderRegion();
})
</script>