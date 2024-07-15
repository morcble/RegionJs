<div id="REGION" class="hidden" style="background: darkslategray;">
	<div class="header-tab flex-container flex-jus-sp flex-align-c them">
		<div class="header-back"><img src="app/images/back-icon.png" alt=""></div>
	</div>
	<iframe class="holder" style="text-align:center;width: 100%;
		height: 100%;"></iframe>
</div>


<script type="text/javascript">
	var REGION = {
		afterRenderData:function(){
			var region=this;
			var url=region.paraMap.get("url");
			region.find(".holder").attr("src",url);
		}
	};


	RegionUtil.ready(function(){
		var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
		staticRegion.afterRenderData = REGION.afterRenderData;
		staticRegion.renderRegion();
	})
</script>