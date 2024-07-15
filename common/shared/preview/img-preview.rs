<style type="text/css">
#REGION .v-center{
  	height:100%;
  	display: inline-flex;
  	align-items: center; 
}
</style>
<div id="REGION" class="hidden">
	<div class="preview-holder v-center"  style="overflow: auto;"><img class="preview" style="max-width: 100%;"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var imgBase64 = this.paraMap.get("imgBase64");
			this.find(".preview").attr("src",imgBase64);
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>