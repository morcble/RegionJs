<style type="text/css">

	#REGION .logo.minimized >.short{
		display:block !important;
	}

	#REGION .logo.minimized >.full{
		display:none;
		
	}

	#REGION .logo>.short{
		display:none;
	}

#REGION .background-logo{
	display: flex;
	align-items: center;
	justify-content: center;
}



   

</style>
<div id="REGION" class="hidden">
	<div class="logo background-logo ">
		<div class="short shotSystemName"></div>
		<div class="full systemName">
			<img src="" alt="" class="systemImage">
		</div>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			//var paraName = this.paraMap.get("paraName");
			var curRegion = this;
			curRegion.minimize=function(){
				curRegion.find(".logo").addClass("minimized");
			}
			
			curRegion.restore=function(){
				curRegion.find(".logo").removeClass("minimized");
			}

			var loginInfo = RegionUtil.getCookie("_region_login");

			if(loginInfo!=null){
				try{
					loginInfo = JSON.parse(loginInfo);
					let systemInfoPromise = SaasUtil.getSystemInfo();
					// let theme =themeUtil.themeStyle
					systemInfoPromise.done(function(systemInfo){
						if(systemInfo.name=="Region Cloud Service"){
							curRegion.find(".systemImage").css("display","block");
							themeUtil.changeLogo(curRegion)
						}else{
							curRegion.find(".systemName").text(systemInfo.name);
							curRegion.find(".systemImage").css("display","none");
						}
						curRegion.find(".shotSystemName").text(systemInfo.name.substring(0,1));
					})
				}
				catch(e){
					console.log(e)
				}
			}
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION");
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.renderRegion();
})
</script>

