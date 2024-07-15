<style type="text/css">
#REGION .msgHolder{
	background: linear-gradient(to left, #7b4397, #2196f3);
	color: white;
	padding: 0.1rem;
	border-radius: 4px;
}

</style>

<div id="REGION" class="hidden">
	<div class="msgHolder"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var msg = this.paraMap.get("msg");
			this.find(".msgHolder").text(msg);
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>