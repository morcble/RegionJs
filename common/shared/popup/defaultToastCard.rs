<style>
    #REGION{
    	padding: 0.8rem 1rem 0.6rem 0.8rem;
        border-radius: 0.2rem;
    }
    
    #REGION>.card-header{
    	padding: 0rem 1.3rem 0rem 1.7rem;
    	position:relative;
    }
    
    #REGION>.card-header>.card-icon{
    	position: absolute;
    	left: 0px;
    	top: 0px;
    }
    
    #REGION>.card-header>.card-title{
	    text-overflow: ellipsis;
	    max-height: 8rem;
	    overflow: auto;
	    width: 100%;
	    display: inline-block;
	    word-break: break-word;
    }
    
    #REGION>.card-header>.card-btn{
    	display: inline-block;
    }
    
    #REGION>.card-header>.close-btn{
    	position: absolute;
    	right: 0px;
    	top: 0px;
    }
    
   #REGION>.card-header .copied{
   		font-size: 0.8rem;
   		color: blue;
   		padding-left: 0.5rem;
   		display: inline-block;
   }
</style>
<div id="REGION" class="hidden background-ff" >
	<div class="card-header">
		<div class="card-icon"><span region-attr="icon" escapeHtml="false"></span></div>
		<div class="card-title"><span region-attr="msg" class="msg" escapeHtml="false"></span><span class="copied hidden">已复制</span></div>
		<div class="close-btn"><i class="fa-regular fa-close"></i></div>
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
			
			curRegion.find(".card-title").click(function(){
				RS.copy($(this).children(".msg").text());
				$(this).children(".copied").removeClass("hidden");
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



