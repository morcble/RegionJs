var themeUtil={
	themeArray:["theme-light","theme-blue","theme-s1","theme-s2","theme-s3","theme-s4","theme-s5","theme-s6","theme-s7","theme-dark"],
    themeStyle:null,
    theme:function (themeIndex){
    	if(themeIndex==null){
    		themeIndex = Number(localStorage.getItem("theme"));
    	}

    	themeIndex = parseInt(themeIndex);
    	if(themeIndex==null||themeIndex==undefined||isNaN(themeIndex))themeIndex=0;

        
        var bodyObject = $(document.body);
        bodyObject.addClass(themeUtil.themeArray[themeIndex]);
        
        for(var i = 0 ; i <themeUtil.themeArray.length;i++){
        	if(i!=themeIndex){
        		bodyObject.removeClass(themeUtil.themeArray[i]);
        	}
        }
        
      
        localStorage.setItem("theme",themeIndex)
        themeUtil.themeStyle=themeIndex
    },
    changeLogo:function (adminLogoRegion){
        if(themeUtil.themeStyle==0){
        	adminLogoRegion.find(".spImage").attr("src","sp/rs/login/images/logo.png")
        }else{
        	adminLogoRegion.find(".spImage").attr("src","sp/rs/login/images/logo-fff.png")
        }
    },
    onloadTheme:function (curRegion){
        let checkedVale=JSON.parse(localStorage.getItem("tableBorder")) || ''
        if(checkedVale){
            if(checkedVale.checkborder){
                curRegion.find(".table-body").attr("border","")
            }else{
                curRegion.find(".table-body").removeAttr("border","")
            }if(checkedVale.checkcenter){
                curRegion.find(".table-body").addClass("text-center")
            }else{
                curRegion.find(".table-body").removeClass("text-center")
            }
            if(checkedVale.checkstripe){
                curRegion.find(".table-body").attr("stripe","")
            }else{
                curRegion.find(".table-body").removeAttr("stripe","")
            }
        }
    }
}
