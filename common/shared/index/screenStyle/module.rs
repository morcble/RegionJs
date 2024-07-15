<style>
    #REGION .fa-navicon{
        font-size: 1.75rem;
    }

</style>
<div id="REGION" class="hidden">
    <header class="background-header">
<!--        <div class="header_control">-->
<!--            <ul>-->
<!--                <li><i class="fa-solid fa-navicon fa-lg"></i></li>-->

<!--            </ul>-->
<!--        </div>-->
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
        //     console.log(headerRegion)
        //     console.log(logoRegion)
        //     console.log("mainRegion")
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
        // },
        setModuleItems:function(navArray){//横向导航
            var curRegion = this;
            let nav_length=  navArray.length;
            let htmlStr="";
            navArray.map(item=>{
                htmlStr +=`<li class="nav_${item.value}"  onclick="REGION.clickActive(event,'${item.value}')">${item.label}</li>`
            })
            RegionUtil.setInnerHtml(curRegion,curRegion.find(".nav-list")[0],htmlStr);
        },
        clickActive:function (event,screenName){

            let targetElem=RegionUtil.getEventTarget(event)
            //$(targetElem).addClass("nav-active").siblings().removeClass("nav-active")
            var map=new HashMap();
            //map.put("screenName",screenName);
            RegionUtil.getRegionById("mainscreen").loadScreen(screenName,map);
        }

    };


    RegionUtil.ready(function(){
        var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
        staticRegion.beforeRenderData = REGION.beforeRenderData;
        staticRegion.afterRenderData = REGION.renderContent;
        staticRegion.setNumber=REGION.setNumber;
        staticRegion.setModuleItems=REGION.setModuleItems;
        // staticRegion.minimize=REGION.minimize;
        // staticRegion.restore=REGION.restore;
        staticRegion.renderRegion();
        staticRegion.show=false;
    })
</script>

