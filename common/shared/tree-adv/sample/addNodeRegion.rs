<style type="text/css">


</style>

<div id="REGION" class="hidden">
		<input region-attr="spId" type="hidden" class="form-control region-editable">
		<input region-attr="module" type="hidden" class="form-control region-editable">
		<input region-attr="pid" type="hidden" class="form-control region-editable">
		<input region-attr="depth" type="hidden" class="form-control region-editable">

	<input type="button" value="添加节点">
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			
		}
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION",null);
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
})
</script>