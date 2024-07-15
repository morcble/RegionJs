
<style type="text/css">

</style>

<div id="REGION" class="hidden">
浮动查询举例
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			this.onSearchDialog = REGION.onSearchDialog;
		},
		onSearchDialog:function(para,inputRegion){
			console.log(para)
			console.log(inputRegion)
		}
		
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
})
</script>
