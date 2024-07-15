<style type="text/css">


</style>

<div id="REGION" class="hidden">
step3
</div>	
<script type="text/javascript">
var REGION = {
		pre:function(){
			//var paraName = this.paraMap.get("paraName");
			console.log("pre form 3")
		},
		next:function(){
			//var paraName = this.paraMap.get("paraName");
			console.log("submit")
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
	formRegion.nextLabel = "提交";
	
	formRegion.onSave = REGION.onSave;
	formRegion.onShow = REGION.onShow;
	formRegion.renderRegion();
})
</script>