<style type="text/css">

</style>

<div id="REGION" class="hidden">
	<div class="form-body" label-position="left" border>
	
			<div class="col-xs-6 col-md-6 form-item-public is-required">
				<message key="Date" class="form-label"></message>
				<input type="text" class="date form-control" region-attr="attr1" format="YYYY-MM" locale="cn">
			</div>	
			<div class="col-xs-6 col-md-6 form-item-public is-required">
				<message key="Time" class="form-label"></message>
				<input type="text" class="time form-control" region-attr="attr2" format="HH:mm">
			</div>
			
			
		
			
			<div class="col-xs-6 col-md-6 form-item-public is-required">
				<message key="Date" class="form-label"></message>
				<input type="text" class="date daterange form-control" region-attr="attr3" format="YYYY-MM-DD" locale="cn">
			</div>
			
			<div class="col-xs-6 col-md-6 form-item-public is-required">
				<message key="Date" class="form-label"></message>
				<input type="text" class="date datetime form-control" region-attr="attr4" dateFormat="YYYY-MM" timeFormat="HH:mm" locale="cn">
			</div>

			<button onclick="REGION.openWindow(event)">aaa</button>
		</div>		
	
						
</div>	


<script type="text/javascript">
var REGION = {
		openWindow:function(event){
			openModalWindow("common/shared/date/samples/datetag.rs");
		},
		afterRenderData:function(){
			$(window).scroll(function(event){
				console.log("scroll")
			});
			
			REGION.count = 0;
			
			var func = function(){
				AlertPlugin.prompt("测试标题"+REGION.count,"测试消息"+REGION.count)
				REGION.count ++;
				if(REGION.count<10){
					setTimeout(func,1000);
				}
			}
			
			setTimeout(func,1000);
		}
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION",null);
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
})
</script>