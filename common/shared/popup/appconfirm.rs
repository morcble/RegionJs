<div id="REGION" class="hidden" style="text-align: center;">
	<div class="alert-content"><span style="position: relative;top: 18%" region-attr="msg"></span></div>
	<div style="position: absolute;bottom: 8px;display: flex;width: 100%; height: 2.3rem;">
		<div style="width: 50%;">
			<button class="btn"  data-type="primary" normal style="width: 50%;height: 80%;font-size: 0.9rem;" onclick='AlertPlugin.yes(event)'>是</button>
		</div>
		<div style="width: 50%;">
			<button class="btn"  data-type="default" normal style="width: 50%;height: 80%;font-size: 0.9rem;" onclick='AlertPlugin.no(event)'>否</button>
		</div>
	</div>
	
	
</div>	
<script type="text/javascript">
RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.renderRegion();
})
</script>