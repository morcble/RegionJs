<div id="REGION" class="hidden" style="background: darkslategray;padding-top:1rem;">
	<iframe class="holder"></iframe>
</div>


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var region=this;
			var url=region.paraMap.get("url");
			region.find(".holder").attr("src","common/shared/preview/rich-preview.html?url="+url);
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>