<style type="text/css">


</style>

<div id="REGION" class="hidden">
	<iframe class="pdfview match-parent"></iframe>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var pdfPage = this.paraMap.remove("pdfPage");
			this.find(".pdfview").attr("src",RegionUtil.appendParaMapToUrl(pdfPage,this.paraMap));
		}
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
})
</script>
