<style>
    #REGION{
    	padding: 0.8rem 1rem 1rem 0.8rem;
        border-radius: 0.2rem;
    }
    
    #REGION>.card-header{
    	padding: 0rem 1.3rem 0.5rem 1.7rem;
    	position:relative;
    }
    
    #REGION>.card-header>.card-icon{
    	position: absolute;
    	left: 0px;
    	top: 0px;
    }
    
    #REGION>.card-header>.card-title{
    	width:100%;
    	display: inline-block;
    	overflow: hidden;
	    text-overflow: ellipsis;
	    white-space: nowrap;
    }
    
    #REGION>.card-header>.card-btn{
    	display: inline-block;
    }
    
    #REGION>.card-header>.close-btn{
    	position: absolute;
    	right: 0px;
    	top: 0px;
    }
</style>
<div id="REGION" class="hidden background-ff" >
	<div class="card-header">
		<div class="card-icon"><span region-attr="icon" escapeHtml="false"></span></div>
		<div class="card-title"><span region-attr="title" escapeHtml="false"></span></div>
		<div class="close-btn"><i class="fa-regular fa-close"></i></div>
	</div>
	<div class="card-body">
		<span region-attr="msg" escapeHtml="false"></span>
	</div>
</div>
<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			var duration = curRegion.paraMap.get("duration");
			curRegion.find(".close-btn").click(function(){
				var promptHolderRegion = RS.getOuterRegion(curRegion);
				promptHolderRegion.closeCard(curRegion.regionId);
			})
			if(duration!="-1"){
				try{
					duration = parseInt(duration)
				}
				catch(e){
					duration = 5000;
				}
				setTimeout(function(){
					var promptHolderRegion = RS.getOuterRegion(curRegion);
					if(promptHolderRegion!=null)promptHolderRegion.closeCard(curRegion.regionId);
				},duration);
			}
		}
}


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
});

</script>



