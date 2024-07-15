<style>
</style>
<div id="REGION" class="hidden">
    <header class="background-header">
        <div class="header-top">
            <ul class="nav-list flex-content flex-jus-sa">
            </ul>
        </div>

        <div class="header_info">

        </div>
    </header>
</div>
<script type="text/javascript">
    var REGION = {
        beforeRenderData:function(){
        },
        renderContent:function(){
			let curRegion = this;
            
            var systemCode = SaasUtil.getSystemCode();
			var loadAdminLogPromise = RS.loadRegion(curRegion.find(".header_info"),"sys/"+systemCode+"/header/headerInfo.rs");
			loadAdminLogPromise.done(function(loadedRegion){
				if(loadedRegion==null){
					RS.loadRegion(curRegion.find(".header_info"),"common/shared/index/common/headerInfo.rs");
				}
				
			})


        },
        // minimize:function(){//页面框架缩小
        // 	let curRegion = this;
        // 	var headerRegion = RS.getOuterRegion(curRegion);
        // 	var logoRegion = headerRegion.logoRegion;
        // 	var mainRegion = RS.getOuterRegion(headerRegion);
        //     mainRegion.minimize();
        //     logoRegion.minimize();
        //     curRegion.find(".fa-navicon").removeClass("rotate-class3").addClass("rotate-class2");
        //     window.minimizedAdmin = true;
        // },
        // restore:function(){//页面框架还原
        // 	let curRegion = this;
        // 	var headerRegion = RS.getOuterRegion(curRegion);
        //  	var logoRegion = headerRegion.logoRegion;
        //  	var mainRegion = RS.getOuterRegion(headerRegion);
        //     mainRegion.restore();
        //     logoRegion.restore();
        //     curRegion.find(".fa-navicon").removeClass("rotate-class2").addClass("rotate-class3");
        //     window.minimizedAdmin = false;
        // }
    };


    RegionUtil.ready(function(){
        var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
        staticRegion.beforeRenderData = REGION.beforeRenderData;
        staticRegion.afterRenderData = REGION.renderContent;
        staticRegion.setNumber=REGION.setNumber;
//         staticRegion.setModuleItems=REGION.setModuleItems;
        staticRegion.minimize=REGION.minimize;
        staticRegion.restore=REGION.restore;
        staticRegion.renderRegion();
        staticRegion.show=false;
    })
</script>

