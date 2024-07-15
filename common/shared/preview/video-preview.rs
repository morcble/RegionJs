<div id="REGION" class="hidden" style="background: darkslategray;padding:3rem 1rem 2rem 1rem; overflow: auto;">
	<div style="height: 100%;position: absolute;right: 1rem;top:0rem">
		 <span class="fa fa-window-maximize popresize" aria-hidden="true" onclick="RegionUtil.toggleModalWindow()" style="color:white;-webkit-text-stroke:2px darkslategray"></span>
		 <span class="fa-solid fa-remove popclose" aria-hidden="true" onclick="RegionUtil.closeModalWindow()" style="color:white;-webkit-text-stroke:4px darkslategray"></span>
	</div>
	<div class="preview-holder"><video class="preview" style="max-width: 100%;max-height: 100%;"  controls="controls"></video></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var url = this.paraMap.get("url");
			this.find(".preview").attr("src",url);
			
			var curRegion = this;
			
			setTimeout(function(){
				curRegion.find(".preview").css("height",curRegion.getRegionDivContainer().clientHeight-96+"px");
			})
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.onRelease = REGION.onRelease;
	staticRegion.renderRegion();
})
</script>