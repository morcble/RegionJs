

<style type="text/css">
    #REGION{
        padding-bottom: 0;
    }

    #REGION .left-active{
    background: #0c2f6f;
    color: #fff;
}
#REGION .form-body{
    width: 500px;
    position: relative;
}
</style>

<div id="REGION" class="hidden">
    <div class="se">
        <span onclick="REGION.changeFrom(event,'default')">默认样式</span>
        <span onclick="REGION.changeFrom(event,'right')">右对齐</span>
        <span onclick="REGION.changeFrom(event,'top')">顶部对齐</span>
        <span onclick="REGION.changeFrom(event,'border')">border属性</span>
    </div>
    <div class="form-body" >
        <div class="col-xs-12 col-md-12 form-item-public">
            <message key="默认显示" class="form-label"></message>
            <input type="text" region-attr="" class="form-control">
        </div>
        <div class="col-xs-12 col-md-12 form-item-public is-required">
            <message key="默认显示" class="form-label"></message>
            <input type="text" region-attr="" class="form-control">
        </div>
        <div class="col-xs-12 col-md-12 form-item-public is-required">
            <message key="默认显示" class="form-label"></message>
            <input type="text" region-attr="" class="form-control">
        </div>
        <div class="col-xs-12 col-md-12 form-item-public is-required">
            <message key="默认最多显示9个字" class="form-label"></message>
            <input type="text" region-attr="" class="form-control">
        </div>
        <div class="col-xs-12 col-md-12 form-item-public is-required">
            <message key="默认最多显示9个字" class="form-label"></message>
            <input type="text" region-attr="" class="form-control">
        </div>
        <div class="col-xs-12 col-md-12 text-center ">
            <button class="btn"  data-type="primary" normal >保存</button>
            <button class="btn"  data-type="default" normal>重置</button>
        </div>
    </div>
</div>


<script type="text/javascript">
    var REGION = {
        afterRenderData: function () {
        },
        changeFrom:function (event,item){
            var curRegion = RS.getRegionByEvent(event);
            let targetElem=RS.getEventTarget(event);
            $(targetElem).addClass('left-active').siblings().removeClass('left-active')
            let target=curRegion.find(".form-body")[0]
            if(item=='border'){
                $(target).attr('border',true)
            }if(item=='default'){
                $(target).removeAttr('border','')
                $(target).removeAttr('label-position','')
            }else{
                $(target).attr('label-position',item)

            }
        }
    };


    RegionUtil.ready(function(){
        var formRegion = RegionUtil.newFormRegion("#REGION");
        formRegion.afterRenderData = REGION.afterRenderData;
        formRegion.renderRegion();
    })
</script>
