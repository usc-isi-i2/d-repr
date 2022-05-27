import * as React from "react";

import {
  Index,
  NDimData,
  DataSlice,
  ContinuousRangeDataSlice,
  PortionOfData,
  UnitOfData
} from "../../../models/data";
import TblCell from "./TblCell";
import { Table } from "antd";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import { CellViewPortPosition } from "./CellViewportPosition";
import { DataView2D } from "./DataView2D";
import { Slice } from "src/models";
import * as _ from "lodash";

const styles = {
  root: {
    "& ul.ant-pagination.ant-table-pagination": {
      float: "left" as "left",
      marginLeft: 8
    }
  }
};

export const defaultProps = {
  onUpdateSelectedSlices: (dataview: DataView2D) => {
    /* void */
  },
  enableSelection: false
};

interface Props
  extends WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  // all data
  data: NDimData;
  dataview: DataView2D;
  onUpdateDataSlices: (
    dslice: DataSlice,
    dataview: DataView2D
  ) => Promise<void>;
  enableSelection: boolean;
}

interface State {
  rowsPerPage: number;
  page: number;
}

class Matrix extends React.Component<Props, State> {
  public static defaultProps = defaultProps;
  public state: State = {
    rowsPerPage: 10,
    page: 0
  };

  public render() {
    const [dataSource, columns] = this.getAntDTableByDataView(
      this.props.enableSelection,
      this.props.dataview
    );
    const pagination = this.getAntDPagination(
      this.state.page,
      this.state.rowsPerPage
    );

    return (
      <div data-testid="data-matrix">
        <Table
          className={this.props.classes.root}
          dataSource={dataSource}
          columns={columns}
          showHeader={false}
          bordered={true}
          rowKey="id"
          pagination={pagination}
          components={{ body: { cell: TblCell } }}
        />
      </div>
    );
  }

  // compute the sliced data for antd table
  private getAntDTableByDataView(
    enableSelection: boolean,
    dataview: DataView2D
  ) {
    const dataSource = [];
    const columns: any = [];
    const unboundedDim0SliceIdx = dataview.unboundDims[0].sliceIdx;
    const unboundedDim1SliceIdx = dataview.unboundDims[1].sliceIdx;

    // TODO: fix me! this is not a general solution, as different part of a data view
    // may have different structures
    const cellIndex = [];
    for (const [sptr, d] of dataview.dslice.iterDFS()) {
      if (sptr instanceof ContinuousRangeDataSlice) {
        cellIndex.push(-1);
      } else {
        cellIndex.push(sptr.toIndex());
      }
    }

    const dim1Values = Array.from(dataview.iterUnboundDimensionValues(1));
    for (const rowIdx of dataview.iterUnboundDimensionValues(0)) {
      const row = [];
      for (const colIdx of dim1Values) {
        cellIndex[unboundedDim0SliceIdx] = rowIdx;
        cellIndex[unboundedDim1SliceIdx] = colIdx;
        const cell = this.props.data.pod.getData(cellIndex);

        let status: "default" | "selected" = "default" as "default";
        if (enableSelection && dataview.isSelected(rowIdx, colIdx)) {
          status = "selected" as "selected";
        }

        row.push({ cell, status });
      }

      dataSource.push({
        id: rowIdx,
        row
      });
    }

    let i = 0;
    const vp: [number, number] = [dataSource.length, dataSource[0].row.length];

    for (const colIdx of dim1Values) {
      columns.push({
        dataIndex: `row.${i}.cell.value`,
        onCell: ((cidx: number) => (row: any, ridx: number) => {
          return {
            enableSelection,
            unboundedDim0SliceIdx,
            unboundedDim1SliceIdx,
            value: row.row[cidx].cell,
            status: row.row[cidx].status,
            onCellClick: this.onCellClick,
            vp: new CellViewPortPosition([ridx, cidx], vp)
          };
        })(i)
      });

      i++;
    }

    return [dataSource, columns];
  }

  // compute the pagination for antd table
  private getAntDPagination(page: number, rowsPerPage: number): object {
    const nPage = this.props.dataview.getNRows();

    return {
      onChange: this.handleChangePage,
      onShowSizeChange: this.handleChangeRowsPerPage,
      total: nPage || 1e6, // using a big number for unknown total items
      showQuickJumper: true,
      showSizeChanger: true,
      current: page + 1,
      pageSize: rowsPerPage,
      showTotal: this.getNPageString
    };
  }

  // handle when a user click on a cell
  private onCellClick = (indice: Index[]) => {
    const slices = this.props.dataview.click(indice);
    this.props.onUpdateSelectedSlices(
      new DataView2D(
        this.props.dataview.data,
        this.props.dataview.dslice,
        this.props.dataview.unboundDims,
        slices
      )
    );
  };

  // handle when a user change to a page
  private handleChangePage = (page: number, pageSize: number) => {
    // from antd so the index start from 1, so we have to deduct it to convert back to our page index
    page -= 1;
    const dslice = this.props.dataview.getRowsView(
      page * pageSize,
      (page + 1) * pageSize
    );

    this.props
      .onUpdateDataSlices(
        dslice,
        new DataView2D(
          this.props.dataview.data,
          dslice,
          this.props.dataview.unboundDims,
          this.props.dataview.selectedSlices
        )
      )
      .then(() => {
        this.setState({ page, rowsPerPage: pageSize });
      });
  };

  // handle when a user change number of rows per page
  private handleChangeRowsPerPage = (
    currentPageSize: number,
    newPageSize: number
  ) => {
    const dslice = this.props.dataview.getRowsView(
      this.state.page * newPageSize,
      (this.state.page + 1) * newPageSize
    );
    this.props
      .onUpdateDataSlices(
        dslice,
        new DataView2D(
          this.props.dataview.data,
          dslice,
          this.props.dataview.unboundDims,
          this.props.dataview.selectedSlices
        )
      )
      .then(() => {
        this.setState({ rowsPerPage: newPageSize });
      });
  };

  // show a readable string of number of items in the matrix
  private getNPageString(nPage: number | undefined) {
    return nPage === undefined
      ? "Unknown number of items"
      : `Total ${nPage} items`;
  }
}

export default injectStyles(styles)(Matrix);
