<style type="text/css">


</style>

<div id="REGION" class="hidden"> 
		<div class="form-body" label-position="top" border>
			<div class="col-xs-12 col-md-12 form-item-public is-required">
				<message key="url" class="form-label"></message>
				<input region-attr="compUrl" type="text" class="form-control" placeholder="http(s)://" maxlength="1000">
			</div>
			
			<div class="col-xs-12 col-md-12 text-center footer">
				<button class="btn" data-type="primary" normal onclick="REGION.useUrl(this)"><message key="global_msg.confirm"></message></button>
			</div>
		</div>
</div>	


<script type="text/javascript">
var REGION = {
		useUrl:function(elem){
			var curRegion = RS.getRegionByElem(elem);
			var valid = curRegion.validate();
			if(valid){
				var openerRegion = RS.getRegionById(curRegion.paraMap.get("openerId"));
				openerRegion.loadRemoteRegion(curRegion.proxy.compUrl);
				RS.closeModalWindow();
			}
		},
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			
		}
};


RS.ready(function(){
	var res = {
			message:{
				dft:{
					url:"组件地址"
				}
			}
	};
	var region = RS.newRegion("#REGION",res);
	region.addValidator("compUrl",new Array(emptyReg),new Array(global_msg.mandatory_fields));
	region.afterRenderData = REGION.afterRenderData;
	region.renderRegion();
})
</script>