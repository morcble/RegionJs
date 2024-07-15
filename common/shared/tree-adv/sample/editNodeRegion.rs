<style type="text/css">


</style>

<div id="REGION" class="hidden">
	<input type="button" value="编辑节点">
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newFormRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>