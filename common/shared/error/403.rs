<style type="text/css">


</style>

<div id="REGION" class="hidden">
	<div>
		<h1>403 No auth access this resource</h1>
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