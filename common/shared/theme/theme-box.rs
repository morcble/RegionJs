<style type="text/css">
#REGION {
	position: relative;
}

#REGION>.box-tabs{
	width: 1000000rem;
	height: 100%;
	position: absolute;
	left: 0;
	top: 0;
    display: flex;
    flex-wrap: wrap;
}

#REGION>.box-tabs>.box-tab{
	display: inline-block;
	height: 100%;
	vertical-align: top;
	padding: 1rem;

}

#REGION .system-entry-holder{
	display: inline-block;
	vertical-align: top;
    
}
#REGION .system-entry-holder > img{
    width: 100%;
    height: auto;
    border: 1px solid #f2f2f2;
    cursor: pointer;
}
#REGION .tab-contorl{
    width: 100%;
    height: 2rem;
    display: flex;
    justify-content: center;
    position: absolute;
    bottom: 1rem;
}
#REGION .tab-box{
    display: flex;
    justify-content: center;
}

#REGION .theme-name{
    width: 100%;
    font-size: 1.2rem;
    padding:1.5rem 0 0 0;
    display: flex;
    justify-content: center;align-items: center;
}
#REGION .theme-title{
    padding-right: 1rem;
}
#REGION .prev-page{
    width: 5rem;
    height:5rem;
    display: flex;
    align-items: center;
    background:rgba(0,0,0,0.2);
    justify-content: center;
    top: 40%;
    transform: translate(0,-40%);
    z-index: 111;
    left: 1.25rem;
    cursor: pointer;
    position: absolute;
    border-radius: 50%;
    font-size: 2rem;
    color: #fff;
}
#REGION .prev-page:hover{
    background:rgba(0,0,0,0.7);
}
#REGION .next-page{
    width: 5rem;
    height:5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
    background:rgba(0,0,0,0.2);
    top: 40%;
    transform: translate(0,-40%);
    z-index: 111;
    right: 1.25rem;
    cursor: pointer;
    border-radius: 50%;
    font-size: 2rem;
    color: #fff;
}
#REGION .next-page:hover{
    background:rgba(0,0,0,0.7);
}
#REGION .prev-page > i{
    transform: rotate(90deg);
}
#REGION .next-page > i{
    transform: rotate(-90deg);
}
#REGION .theme-checkbox{
    position: absolute;
    top: 33.3rem;
    right: 1.4rem;

    height: 2rem;
    z-index: 999;
}
#REGION .theme-checkbox > label{
    display: flex;
    align-items: center;
    line-height: 1.125rem;
}
#REGION .theme-checkbox > label > span{
    padding-left: 0.6rem;
    display: block;
    cursor: pointer;
}
/*#REGION .skin-pf{*/
/*    position: absolute;*/
/*    bottom: 5rem;*/
/*    left: 55%;*/
/*    z-index: 9999;*/
/*}*/
</style>

<div id="REGION" class="hidden">
<!--    <div class="skin-pf"><button data-type="primary" class="btn" plain onclick="REGION.clickTheme(event)">应用皮肤</button></div>-->
    <div class="prev-page" onclick="REGION.prePage(event)"><i class="fa-solid fa-chevron-down"></i></div>
    <div class="next-page" onclick="REGION.nextPage(event)"><i class="fa-solid fa-chevron-down"></i></div>
        <div class="theme-checkbox">
            <label>
                <input type="checkbox" class="autoSwitch" checked="checked">
                <span>自动切换</span>
            </label>
        </div>
		<div class="box-tabs">

        </div>
        <div class="tab-contorl">

        </div>
</div>


<script type="text/javascript">
var REGION = {
        pageNo:0,
		afterRenderData:function(){
			var curRegion = this;
			var columns = parseInt(curRegion.paraMap.get("columns"));
			var rows = parseInt(curRegion.paraMap.get("rows"));
			//var show =curRegion.paraMap.get("show");
            //curRegion.show=show;
			curRegion.columns = columns;//第一页
			curRegion.rows = rows;//第一页
			curRegion.pageNo = 0;//第一页
			curRegion.pageSize = columns*rows;
            themeUtil.theme();
		},
        renderSystemEntry:function(systemArray){
            var curRegion = this;
            setTimeout(function(){
                curRegion.viewWidth = $(curRegion.getRegionDivElem()).width();
                REGION.subRenderSystemEntry(curRegion,systemArray);
            })
        },
		subRenderSystemEntry:function(curRegion,systemArray){
			var dynamicHtml = "";
			var pageCount = Math.ceil(systemArray.length / (curRegion.rows*curRegion.columns));
			curRegion.pageCount = pageCount;//总页数
			var tabBoxHtml = '';
			st:for(var i = 0 ; i <pageCount;i++){
				tabBoxHtml+= '<div class="box-tab">';
				for(var j=0;j<curRegion.pageSize;j++){
					var systemDataIndex = i*curRegion.pageSize + j;
					if(systemDataIndex>(systemArray.length-1))
						break st;
					var systemInfo = systemArray[systemDataIndex];
                    //css样式规范用列 不需要做事件操作
                   // if(curRegion.show==1){
                     //   tabBoxHtml += '<div class="system-entry-holder"  style="width:'+Math.round(100/curRegion.columns)+'%;height:'+Math.round(100/curRegion.rows)+'%"><img src="'+systemInfo.src+'" alt=""><div class="theme-name color-fff"><div class="theme-title">'+systemInfo.name+'</div><div></div></div></div>';

                   // }else{
                        tabBoxHtml += '<div class="system-entry-holder" onclick="REGION.clickTheme(event,'+systemInfo.type+')"  style="width:'+Math.round(100/curRegion.columns)+'%;height:'+Math.round(100/curRegion.rows)+'%"><img src="'+systemInfo.src+'" alt=""><div class="theme-name color-fff"><div class="theme-title">'+systemInfo.name+'</div><div><button data-type="primary" class="btn hidden" plain onclick="REGION.clickTheme(event,'+systemInfo.type+')">应用皮肤</button></div></div></div>';

                   // }
				}
				tabBoxHtml += '</div>';
			}
			RS.setInnerHtml(curRegion,curRegion.find(".box-tabs")[0],tabBoxHtml);

			setTimeout(function(){
				curRegion.find(".box-tab").css("width",curRegion.viewWidth+"px");
			})

			var tabControl=''
				tabControl+='<div class="tab-box">';
			for(var i = 0 ; i <pageCount;i++){
                if(i==0){
                    tabControl+='<span class="ctrl-btn ctrl-ban-active" onclick="REGION.switchTab(event,'+i+')"></span>';
                }else{
                    tabControl+='<span class="ctrl-btn" onclick="REGION.switchTab(event,'+i+')"></span>';
                }
			}
			tabControl+='</div>';
			RS.setInnerHtml(curRegion,curRegion.find(".tab-contorl")[0],tabControl);
		},
		switchTab:function(event,targetPageNo,page){
			var curRegion = RS.getRegionByEvent(event);
            var targetElem = RegionUtil.getEventTarget(event);
			if(curRegion.isAnimating)return;
			if(curRegion.pageNo==targetPageNo)return;
			curRegion.isAnimating = true;
			var boxTabs = curRegion.find('.box-tabs');
			var curLeft = parseInt(boxTabs.css("left").replace("px",""));
			var viewWidth = curRegion.viewWidth;
            if(page=='page'){
                var pageElem=curRegion.find(".tab-box > .ctrl-btn")[targetPageNo]
                $(pageElem).addClass("ctrl-ban-active").siblings().removeClass("ctrl-ban-active")
            }else{
                $(targetElem).addClass("ctrl-ban-active").siblings().removeClass("ctrl-ban-active")
            }
			var leftOffset = 0;
			leftOffset = curLeft + (curRegion.pageNo - targetPageNo)*viewWidth+'px';

			boxTabs.animate({left: leftOffset}, 'fast',function(){
				curRegion.isAnimating = false;
			});
			curRegion.pageNo = targetPageNo;
            REGION.pageNo = targetPageNo
		},
		nextPage:function(event){
			var curRegion = RS.getRegionByEvent(event);
			if(curRegion.pageNo==(curRegion.pageCount-1))return;//已经是最后一页
			REGION.switchTab(event,curRegion.pageNo+1,'page');
			
			
			var autoSwitch = curRegion.find(".autoSwitch").prop("checked");
            if(autoSwitch){
                REGION.clickTheme()
            }
		},
		prePage:function(event){
			var curRegion = RS.getRegionByEvent(event);//已经是第一页
			if(curRegion.pageNo==0)return;
			
			REGION.switchTab(event,curRegion.pageNo-1,'page');
			var autoSwitch = curRegion.find(".autoSwitch").prop("checked");
            if(autoSwitch){
                REGION.clickTheme();
            }
		},
    	clickTheme:function (){
            themeUtil.theme(REGION.pageNo);
            var cloudMainRegion = RS.getRegionById("rscloud_main");
            if(cloudMainRegion!=null&&cloudMainRegion.headerRegion!=null&&cloudMainRegion.headerRegion.logoRegion!=null)
            	themeUtil.changeLogo(cloudMainRegion.headerRegion.logoRegion);
    	}
};


RegionUtil.ready(function(){
	var region = RegionUtil.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.renderSystemEntry = REGION.renderSystemEntry;
	region.renderRegion();
})
</script>
