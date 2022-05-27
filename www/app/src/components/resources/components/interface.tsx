import { Slice } from "src/models";

export interface ResourceComponentProps {
  // fire when selected slices was modified inside the resource component
  onUpdateSelectedSlices: (resourceId: string, newRegion: Slice[]) => void;
  onHideResourcePanel: (resourceId: string) => void;
  onDeleteResource: (resourceId: string) => void;
}

export interface ResourceComponent {
  // wait until the resource has finish its initialization
  waitForInit: () => Promise<void>;
  // enable user to select a region in a resource
  enableSelection: () => Promise<void>;
  // not allow user to select a region in a resource
  disableSelection: () => Promise<void>;
  // set selected region in a resource, note that onUpdateSelectedSlices won't be fired
  // when this method is invoked
  setSelectedSlices: (region: Slice[]) => Promise<void>;
}

export const defaultResourceComponentProps = {
  onUpdateSelectedSlices: (resourceId: string, newRegion: Slice[]) => {
    /* do nothing */
  },
  onHideResourcePanel: (resourceId: string) => {
    /* do nothing */
  },
  onDeleteResource: (resourceId: string) => {
    /* do nothing */
  }
};
