<style type="text/css">
    #REGION{
        padding-bottom: 0;
    }

</style>
<div id="REGION" class="hidden">
    <div class="table-body"   border>
        <div class="datas-columns row">
            <div class="col-xs-1 maxwidth60"><message key="global_msg.index"></message></div>
            <div class="col-xs-2"><message key="name"></message></div>
            <div class="col-xs-1 "><message key="status"></message></div>
            <div class="col-xs-2"><message key="global_msg.create_dt"></message></div>
            <div class="col-xs-4 no-scrollbar tableScrollRow">
                <div class="container">
                    <div class="col-xs-6"><message key="description"></message></div>
                    <div class="col-xs-6"><message key="updateBy"></message></div>
                </div>
            </div>
            <div class="col-xs-3 text-center"><message key="global_msg.operation"></message></div>
        </div>
        <div class="datas" region-list="list">
            <div class="row" onclick="RegionUtil.handleListData(event,REGION.choose)">
                <div class="col-xs-1 maxwidth60"><span region-attr="index"></span></div>
                <div class="col-xs-2"><span region-attr="name"></span></div>
                <div class="col-xs-1"><span region-attr="status" region-ds="EnableStatus"></span></div>
                <div class="col-xs-2"><span region-attr="shortCreateDt"></span></div>
                <div class="col-xs-4 no-scrollbar tableScrollRow">
                    <div class="container">
                        <div class="col-xs-6"><span region-attr="description"></span></div>
                        <div class="col-xs-6"><span region-attr="updateBy"></span></div>
                    </div>
                </div>
                <div class="col-xs-3 text-center">
                    <button class="btn"  data-type="primary" plain>解析</button>
                    <button class="btn" data-type="primary" plain><i class="fa-light fa-pencil" title="编辑"></i></button>
                    <button class="btn"   data-type="danger" plain>删除</button>
                </div>
            </div>
        </div>
    </div>

</div>


<script type="text/javascript">
    var REGION = {
        list:[
            {
                createBy:"2100",
                createDt:"2024-02-01 09:27:39",
                createDtAsStr:"2024-02-01 09:27:39",
                description:"每个服务器都有对应的DNS",
                id:"4758672318060822528",
                name:"rs.servers",
                shortCreateDt:"2024-02-01",
                softDelete: 0,
                spId :"100",
                status : 1,
                updateBy: "2100",
                updateDt:"2024-02-01 09:27:39",
            },
            {
                createBy:"2100",
                createDt:"2024-02-01 09:27:39",
                createDtAsStr:"2024-02-01 09:27:39",
                description:"每个服务器都有对应的DNS",
                id:"4758672318060822528",
                name:"rs.servers",
                shortCreateDt:"2024-02-01",
                softDelete: 0,
                spId :"100",
                status : 1,
                updateBy: "2100",
                updateDt:"2024-02-01 09:27:39",
            }
        ],
        beforeRenderData: function () {
            var curRegion=this;
            curRegion.regionData.list=REGION.list
        },
        afterRenderData:function(){
            var curRegion=this;
            if(!curRegion.inited){
                curRegion.inited = true;

                RS.enableScroll(curRegion.find(".table-body"));
            }
        },
        choose:function (rowData,targetElem){
            var rowObj = $(targetElem).closest(".row");
            rowObj.addClass("highlight").siblings().removeClass("highlight");
        }
    };
    RegionUtil.ready(function () {
        var res = {
            message:{
                dft:{
                    spId:"spId",
                    name:"域名",
                    description:"描述",
                    status:"状态",
                    midServerId:"midServerId"
                }
            },

        };
        var gridRegion = RS.newFormRegion("#REGION",res);;
        gridRegion.beforeRenderData = REGION.beforeRenderData;
        gridRegion.afterRenderData = REGION.afterRenderData;
        gridRegion.renderRegion();
    })
</script>


