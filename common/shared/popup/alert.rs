<div id="REGION" class="hidden" style="text-align: center;">
	<div class="alert-content"><span style="position: relative;top: 20%" region-attr="msg"></span></div>
	<button class="btn" data-type="primary" normal onclick='AlertPlugin.hideMsg(event)'><message key="global_msg.ok"></message></button>
</div>	
<script type="text/javascript">
RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.renderRegion();
})
</script>

