/*
 * Copyright (C) 2012 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package android.hardware.input;

import android.view.InputEvent;

interface IInputManager {
    String getVelocityTrackerStrategy();
    void getInputDevice(int deviceId);
    int[] getInputDeviceIds();

    boolean isInputDeviceEnabled(int deviceId);
    void enableInputDevice(int deviceId);
    void disableInputDevice(int deviceId);

    boolean hasKeys(int deviceId, int sourceMask, in int[] keyCodes, out boolean[] keyExists);

    int getKeyCodeForKeyLocation(int deviceId, in int locationKeyCode);

    void tryPointerSpeed(int speed);

    boolean injectInputEvent(in InputEvent ev, int mode);

    boolean injectInputEventToTarget(in InputEvent ev, int mode, int targetUid);
}
