<style type="text/css">


</style>

<div id="REGION" class="hidden">
Hello Region Js !
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			
		}
};


RS.ready(function(){
	var region = RS.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.renderRegion();
})
</script>