<style>

</style>
<div id="REGION" class="hidden" no-footer style="overflow: inherit;">
	<div class="show-message" region-attr="msg"></div>
</div>
<script type="text/javascript">
var REGION = {
	beforeRenderData:function(){
		let region = this;

	},
	showMsg:function (msg,width) {
		let region = this;
		region.find(".show-message").css("max-width",width)
		region.setMsg(msg);
	}
}

RegionUtil.ready(function(){
	var res = {

	};
	var formRegion = RegionUtil.newFormRegion("#REGION",res);
	formRegion.saveSuccessCallBack = REGION.saveSuccessCallBack;
	formRegion.saveFailedCallBack = REGION.saveFailedCallBack;
	formRegion.beforeRenderData = REGION.beforeRenderData;
	formRegion.showMsg = REGION.showMsg;

	formRegion.renderRegion();
		
});  
</script>




