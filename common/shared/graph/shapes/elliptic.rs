<style type="text/css">
#REGION{
	text-align: center;
	display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid black;
    border-radius: 50%;
}

</style>

<div id="REGION" class="hidden">
<!-- onmouseover="REGION.onSelected(this)" -->
	<div class="rect-content">
			<span region-attr="label"></span>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			
		}
};


RS.ready(function(){
	var curRegion = RS.newRegion("#REGION",null);
	curRegion.afterRenderData = REGION.afterRenderData;
	curRegion.renderRegion();
})
</script>