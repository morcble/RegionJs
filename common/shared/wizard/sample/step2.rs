<style type="text/css">


</style>

<div id="REGION" class="hidden">
step2
</div>	
<script type="text/javascript">
var REGION = {
		pre:function(){
			//var paraName = this.paraMap.get("paraName");
			console.log("pre form 2")
		},
		next:function(){
			//var paraName = this.paraMap.get("paraName");
			
		},
		onSave:function(){
			
		},
		onShow:function(stepData){
			//回到当前step的时候
		}
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.pre = REGION.pre;
	formRegion.next = REGION.next;
	formRegion.preLabel = "上一步";
	formRegion.nextLabel = "下一步";
	
	formRegion.onSave = REGION.onSave;
	formRegion.onShow = REGION.onShow;
	formRegion.renderRegion();
})
</script>