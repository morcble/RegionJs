<style type="text/css">
#REGION{
	position:relative;
	padding-left: 50%;
	
	height: 100%;
}
#REGION>.palette{
	width: 50%;
	position: absolute;
	height: 100%;
	background: #f2f2f2;
	left: 0;
	top: 0;
}
#REGION>.content{
	position:relative;
	width: 100%;
	height: 100%;
}
#REGION>.properties{
	width: 20rem;
	position: absolute;
	height: 100%;
	background: blue;
	right: 0;
	top: 0;
}

</style>

<div id="REGION" class="hidden">
	<div class="palette">
	</div>
	<div class="content">
	</div>
<!-- 	<div class="properties"> -->
<!-- 	</div> -->
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			var curRegion = this;
			RS.loadRegion(curRegion.find(".content"),"common/shared/graph/content.rs");
			RS.loadRegion(curRegion.find(".palette"),"common/shared/graph/content.rs");
		}
};


RS.ready(function(){
	var region = RS.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.renderRegion();
})
</script>