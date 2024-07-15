<style type="text/css">


</style>

<div id="REGION" class="hidden">
	<div>
		<h1>404 Resource doesn't exist</h1>
	</div>
	
</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			//var paraName = this.paraMap.get("paraName");
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = new StaticRegion("#REGION","403ERROR");
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.renderRegion();
})

</script>