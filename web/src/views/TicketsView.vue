<!-- Copyright 2023 Zinc Labs Inc.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http:www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License. 
-->

<!-- eslint-disable vue/valid-attribute-name -->
<template>
  <q-page class="q-pa-none" style="min-height: inherit">
    <q-table
      :title="t('ticket.header')"
      :rows="tickets"
      row-key="id"
      :pagination="pagination"
      :filter="filterQuery"
      :filter-method="filterData"
      :loading="loading"
      @row-click="editTicket"
    >
      <template #no-data> <NoData /></template>

      <template #top-right>
        <q-input
          v-model="filterQuery"
          filled
          dense
          class="q-ml-auto q-mb-xs"
          :placeholder="t('ticket.search')"
        >
          <template #prepend>
            <q-icon name="search" />
          </template>
        </q-input>
        <q-btn
          class="q-ml-md q-mb-xs text-bold no-border"
          padding="sm lg"
          color="secondary"
          no-caps
          :label="t(`ticket.add`)"
          @click="addTicket"
        />
      </template>

      <template v-slot:body-cell-#="props">
        <q-td :props="props" width="80" @click="editTicket(e, props)">
          {{ props.value }}
        </q-td>
      </template>
    </q-table>

    <q-dialog
      v-model="showaddTicketDialog"
      position="right"
      full-height
      maximized
    >
      <add-update-ticket @updated="updateTicketList" />
    </q-dialog>

    <q-dialog
      v-model="showUpdateTicketDialog"
      position="right"
      full-height
      maximized
    >
      <add-update-ticket v-model="ticket" @updated="ticketUpdated" />
    </q-dialog>
  </q-page>
</template>

<script lang="ts">
// @ts-nocheck
import { defineComponent, ref } from "vue";
import { useStore } from "vuex";
import { useQuasar, date } from "quasar";
import { useI18n } from "vue-i18n";

import ticketsService from "../services/tickets";
import AddUpdateTicket from "../components/tickets/AddUpdateTicket.vue";
import NoData from "../components/shared/grid/NoData.vue";

export default defineComponent({
  name: "PageTicket",
  components: {
    AddUpdateTicket,
    NoData,
  },
  setup() {
    const store: any = useStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const ticket: any = ref({});
    const tickets: any = ref([]);
    const getTickets: any = () => {
      const dismiss: any = $q.notify({
        spinner: true,
        message: "Please wait while loading tickets...",
      });
      ticketsService.list(0, 100000, "subject", false, "").then((res) => {
        var counter = 1;
        tickets.value = res.data.data.map(
          (data: {
            id: any;
            subject: any;
            description_text: any;
            status: any;
            created_at: string | number | Date;
            updated_at: string | number | Date;
            comments: any;
          }) => {
            return {
              "#": counter++,
              id: data.id,
              subject: data.subject,
              description: data.description_text,
              status: data.status,
              created: date.formatDate(data.created_at, "YYYY-MM-DDTHH:mm:ssZ"),
              updated: date.formatDate(data.updated_at, "YYYY-MM-DDTHH:mm:ssZ"),
              comments: data.comments,
            };
          }
        );

        dismiss();
      });
    };

    getTickets();

    const showaddTicketDialog = ref(false);
    const showUpdateTicketDialog = ref(false);

    const addTicket = () => {
      showaddTicketDialog.value = true;
    };
    const editTicket = (
      event: any,
      props: {
        id: any;
        subject: any;
        description: any;
        status: any;
        createdAt: string | number | Date;
        updatedAt: string | number | Date;
        comments: any;
      }
    ) => {
      ticket.value = {
        id: props.id,
        subject: props.subject,
        description: props.description,
        status: props.status,
        created: date.formatDate(props.createdAt, "YYYY-MM-DDTHH:mm:ssZ"),
        updated: date.formatDate(props.updatedAt, "YYYY-MM-DDTHH:mm:ssZ"),
        comments: props.comments,
      };
      showUpdateTicketDialog.value = true;
    };
    const deleteTicket = (props: { row: { id: string } }) => {
      $q.dialog({
        title: "Delete Ticket",
        message:
          "You are about to delete a ticket: <ul><li>" +
          props.row.id +
          "</li></ul>",
        cancel: true,
        persistent: true,
        html: true,
      }).onOk(() => {
        ticketsService.delete(props.row.id).then(() => {
          getTickets();
        });
      });
    };

    return {
      t,
      loading: ref(false),
      ticket,
      showaddTicketDialog,
      showUpdateTicketDialog,
      tickets,
      addTicket,
      editTicket,
      deleteTicket,
      ticketUpdated() {
        showUpdateTicketDialog.value = false;
        getTickets();
      },
      pagination: {
        rowsPerPage: 20,
      },
      filterQuery: ref(""),
      filterData(rows: string | any[], terms: string) {
        const filtered = [];
        terms = terms.toLowerCase();
        for (let i = 0; i < rows.length; i++) {
          if (
            rows[i]["subject"].toLowerCase().includes(terms) ||
            rows[i]["description"].toLowerCase().includes(terms)
          ) {
            filtered.push(rows[i]);
          }
        }
        return filtered;
      },
    };
  },
  methods: {
    updateTicketList(data: { data: any }) {
      this.showaddTicketDialog = false;
      let ticketdata = [];
      ticketdata.push(data.data);
      ticketdata = ticketdata.map((data) => {
        return {
          "#": this.tickets.length + 1,
          id: data.id,
          subject: data.subject,
          description: data.description_text,
          status: data.status,
          created: date.formatDate(data.created_at, "YYYY-MM-DDTHH:mm:ssZ"),
          updated: date.formatDate(data.updated_at, "YYYY-MM-DDTHH:mm:ssZ"),
          comments: data.comments,
        };
      });

      this.tickets = [...this.tickets, ...ticketdata];

      this.$q.notify({
        type: "positive",
        message: `Ticket added successfully.`,
      });
    },
  },
});
</script>
