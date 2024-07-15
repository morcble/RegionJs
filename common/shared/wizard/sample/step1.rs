<style type="text/css">


</style>

<div id="REGION" class="hidden">
step1
</div>	
<script type="text/javascript">
var REGION = {
		next:function(){
			console.log("next form 1")
//if return null, will proceed to next step ;
//else if return promise. only if promise resolve true, will proceed to next step
		},
		onSave:function(){
			//当前步骤的数据
		},
		onShow:function(stepData){
			//回到当前step的时候
		}
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.next = REGION.next;
	formRegion.nextLabel = "下一步";
	
	formRegion.onSave = REGION.onSave;
	formRegion.onShow = REGION.onShow;
	formRegion.renderRegion();
})
</script>