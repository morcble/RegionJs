<div id="REGION" class="hidden" style="background: darkslategray;padding:0rem; overflow: auto;">
	<div style="height: 100%;position: absolute;right: 0rem;top:0rem">
		<span class="fa-solid fa-remove popclose" aria-hidden="true" onclick="RegionUtil.gotoBack()" style="z-index:100;color:white;-webkit-text-stroke:4px darkslategray"></span>
	</div>
	<div class="preview-holder"><video class="preview" style="max-width: 100%;"  controls="controls"></video></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var url = this.paraMap.get("url");
			this.find(".preview").attr("src",url);
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.onRelease = REGION.onRelease;
	staticRegion.renderRegion();
})
</script>