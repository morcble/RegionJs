<style>

</style>
<div id="REGION" class="hidden">
    <div class="header_nav_wrap">
        <div class="shift-left"><i class="fa-solid fa-angle-left"></i></div>
        <div class="shift-right"><i class="fa-solid fa-angle-right"></i></div>
        <div class="header_nav no-scrollbar">
            <div class="header_nav_inner">
            </div>
        </div>
    </div>
</div>


<script type="text/javascript">
	var REGION = {
		selectItem:function(itemId){
			var foundItem = false;
			var offset = 0;
			this.find(".nav-item").each(function(){
				if(!foundItem){
					if($(this).hasClass("menuId-"+itemId)){
						foundItem = true;
						$(this).addClass("selected");
						return true;
					}
					offset+=$(this).outerWidth(true);
				}

				$(this).removeClass("selected");
				return true;
			});

			this.find(".header_nav").animate({
				scrollLeft: offset
			},100);

		},
		selectItem:function(itemId){
			var foundItem = false;
			var offset = 0;
			this.find(".nav-item").each(function(){
				if(!foundItem){
					if($(this).hasClass("menuId-"+itemId)){
						foundItem = true;
						$(this).addClass("selected");
						return true;
					}
					offset+=$(this).outerWidth(true);
				}

				$(this).removeClass("selected");
				return true;
			});

			this.find(".header_nav").animate({
				scrollLeft: offset
			},100);

		},
		recalculateWidth:function(){
			var headerNavInnerObj = this.find(".header_nav_inner");
			var width = 0;
			headerNavInnerObj.children(".nav-item").each(function(){
				width+=$(this).outerWidth(true);
			});
			width=width+10;
			headerNavInnerObj.css("width",width+"px");
			return width;
		},
		addNavItem:function(tabName,menuId){
			var curRegion = this;
			var headerNavInnerObj = this.find(".header_nav_inner");
			headerNavInnerObj.find(".nav-item").removeClass("selected");

			var itemHtml='<li menu-id='+menuId+' class="selected nav-item menuId-'+menuId+'">';
			itemHtml+='<span class="nav-title">'+tabName+'</span>';
			itemHtml+='<span class="fa-solid fa-remove nav-close"></span>';
			itemHtml+='</li>';
			var itemObj = $(itemHtml);
			RegionUtil.addRegionUniqueId(itemObj[0],this.regionId);

			headerNavInnerObj.append(itemObj);

			itemObj.click(function(event){//切换tab
				var contentRegion = RS.getOuterRegion(curRegion);
				contentRegion.loadTab(this.getAttribute("menu-id"),null,null,true);
			});
			itemObj.find(".nav-close").click(function(event){//关闭tab
				event.stopPropagation();
				var navItem = $(this).closest(".nav-item");
				var menuId = navItem.attr("menu-id");
				navItem.remove();
				var contentRegion = RS.getOuterRegion(curRegion);
				contentRegion.unLoadTab(menuId);
			});
			var region = this;

			//scroll到最后
			setTimeout(function(){
				var maxWidth = region.recalculateWidth();

				maxWidth = maxWidth - region.find(".header_nav").outerWidth();
				region.scrollOffset = maxWidth;

				region.find(".header_nav").animate({
					scrollLeft: region.scrollOffset
				},100);
			});
		},
		renderContent:function(){
            let curRegion = this
			this.recalculateWidth();

			var region = this;
			region.scrollOffset = 0;
			this.find(".shift-right").click(function(){
				region.scrollOffset +=300;
				var maxWidth = region.recalculateWidth();

				maxWidth = maxWidth - region.find(".header_nav").innerWidth();
				if(region.scrollOffset>maxWidth){
					region.scrollOffset = maxWidth;
				}
				region.find(".header_nav").animate({
					scrollLeft: region.scrollOffset
				},100);
			});

			this.find(".shift-left").click(function(){
				region.scrollOffset -=300;
				if(region.scrollOffset<0){
					region.scrollOffset = 0;
				}
				region.find(".header_nav").animate({
					scrollLeft: region.scrollOffset
				},100);
			});

			this.find(".fa-navicon").attr("title","菜单伸缩");
			this.find(".fa-refresh").attr("title","刷新");
			this.find(".fa-cog").attr("title","设置");

			var navIcon = this.find(".fa-navicon");
			navIcon.click(function(){
				var rootLayout = RegionUtil.getRegionById("rootLayout");
				if(navIcon.hasClass("rotate-class2")){
					navIcon.removeClass("rotate-class2").addClass("rotate-class3");
					setTimeout(function(){
						navIcon.removeClass("rotate-class3");
					},200);
					rootLayout.restore();
				}
				else{
					navIcon.removeClass("rotate-class3").addClass("rotate-class2");
					rootLayout.minimize();
				}
			})


		},
		setNavItems:function(navArray){//横向导航
			var curRegion = this;
			let nav_length=  navArray.length
			console.log(navArray)
			let htmlStr="";
			navArray.map(item=>{
				htmlStr +=`<li class="nav-active" style="width:calc( 100% / ${nav_length})" onclick="REGION.clickActive(event,'${item.value}')">${item.label}</li>`
			})
			console.log(htmlStr)
			RegionUtil.setInnerHtml(curRegion,curRegion.find(".nav-list")[0],htmlStr);
		},
		clickActive:function (event,screenName){
			let curRegion=RegionUtil.getRegionByEvent(event)
			//curRegion.find(".nav-active")
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
		staticRegion.setNavItems=REGION.setNavItems;
		
		staticRegion.addNavItem = REGION.addNavItem;
		staticRegion.recalculateWidth = REGION.recalculateWidth;
		staticRegion.selectItem = REGION.selectItem;
		
		staticRegion.renderRegion();
		staticRegion.show=false;
	})
</script>

